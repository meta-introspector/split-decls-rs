macro_rules! deps {
    () => {
        PerfStatsLevel!();
    };
}

macro_rules! set_perf_stats {
    () => {
        deps!();
        # [doc = " Sets the perf stats level for current thread."] pub fn set_perf_stats (lvl : PerfStatsLevel) { unsafe { ffi :: rocksdb_set_perf_level (lvl as c_int) ; } }
    };
}

set_perf_stats!();