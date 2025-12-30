// Generated macro for Counters (struct)
macro_rules! Depcrate_measure_perf_counter_linuxCounters {
() => {
// Module: crate::measure::perf_counter::linux
// Provides: {"Counters"}
// Dependencies: {}
# [doc = " A collection of CPU performance counters."] # [doc = " The counters are optional, because some CPUs are not able to record them."] struct Counters { cycles : Option < Counter > , instructions : Option < Counter > , branch_misses : Option < Counter > , cache_misses : Option < Counter > , cache_references : Option < Counter > , }
};
}
