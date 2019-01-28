use crate::{column_default, Column};
use chrono::{DateTime, Duration, Local};
use failure::Error;
use libc::_SC_CLK_TCK;
use procfs::{Io, ProcResult, Process};
use std::cmp;
use std::collections::HashMap;

// ---------------------------------------------------------------------------------------------------------------------
// StartTime
// ---------------------------------------------------------------------------------------------------------------------

pub struct StartTime {
    pub visible: bool,
    header: String,
    contents: HashMap<i32, String>,
    max_width: usize,
    boot_time: DateTime<Local>,
    sc_clk_tck: i64,
}

impl StartTime {
    pub fn new() -> Self {
        let header = String::from("Start");
        StartTime {
            visible: true,
            contents: HashMap::new(),
            max_width: header.len(),
            header: header,
            boot_time: procfs::boot_time().unwrap(),
            sc_clk_tck: unsafe { libc::sysconf(_SC_CLK_TCK) },
        }
    }
}

impl Column for StartTime {
    fn add(
        &mut self,
        proc: &Process,
        _prev_proc: &Process,
        _prev_io: &ProcResult<Io>,
    ) -> Result<(), Error> {
        let start_time = Duration::seconds(proc.stat.starttime / self.sc_clk_tck);
        let start_time = self
            .boot_time
            .checked_add_signed(start_time)
            .unwrap_or(self.boot_time);
        let content = format!("{}", start_time.format("%Y/%m/%d %H:%M"));

        self.max_width = cmp::max(content.len(), self.max_width);

        self.contents.insert(proc.pid(), String::from(content));
        Ok(())
    }

    column_default!();
}
