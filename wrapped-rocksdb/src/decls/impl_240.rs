macro_rules! deps {
    () => {
        WaitForCompactOptions!();
    };
}

macro_rules! impl_240 {
    () => {
        deps!();
        impl WaitForCompactOptions { # [doc = " If true, abort waiting if background jobs are paused. If false,"] # [doc = " ContinueBackgroundWork() must be called to resume the background jobs."] # [doc = " Otherwise, jobs that were queued, but not scheduled yet may never finish"] # [doc = " and WaitForCompact() may wait indefinitely (if timeout is set, it will"] # [doc = " abort after the timeout)."] # [doc = ""] # [doc = " Default: false"] pub fn set_abort_on_pause (& mut self , v : bool) { unsafe { ffi :: rocksdb_wait_for_compact_options_set_abort_on_pause (self . inner , c_uchar :: from (v)) ; } } # [doc = " If true, flush all column families before starting to wait."] # [doc = ""] # [doc = " Default: false"] pub fn set_flush (& mut self , v : bool) { unsafe { ffi :: rocksdb_wait_for_compact_options_set_flush (self . inner , c_uchar :: from (v)) ; } } # [doc = " Timeout in microseconds for waiting for compaction to complete."] # [doc = " when timeout == 0, WaitForCompact() will wait as long as there's background"] # [doc = " work to finish."] # [doc = ""] # [doc = " Default: 0"] pub fn set_timeout (& mut self , microseconds : u64) { unsafe { ffi :: rocksdb_wait_for_compact_options_set_timeout (self . inner , microseconds) ; } } }
    };
}

impl_240!();