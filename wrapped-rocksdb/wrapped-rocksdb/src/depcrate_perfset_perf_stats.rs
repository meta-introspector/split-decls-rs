// Generated macro for set_perf_stats (function)
macro_rules! Depcrate_perfset_perf_stats {
() => {
// Module: crate::perf
// Provides: {"set_perf_stats"}
// Dependencies: {}
# [doc = " Sets the perf stats level for current thread."] pub fn set_perf_stats (lvl : PerfStatsLevel) { unsafe { ffi :: rocksdb_set_perf_level (lvl as c_int) ; } }
};
}
