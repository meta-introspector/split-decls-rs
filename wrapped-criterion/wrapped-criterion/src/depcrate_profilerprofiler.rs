// Generated macro for Profiler (trait)
macro_rules! Depcrate_profilerProfiler {
() => {
// Module: crate::profiler
// Provides: {"Profiler"}
// Dependencies: {}
# [doc = " Extension trait for external crates to implement which provides start/stop"] # [doc = " hooks when profiling (but not when benchmarking) functions."] pub trait Profiler { # [doc = " This function is called when Criterion.rs starts profiling a particular"] # [doc = " benchmark. It provides the stringified benchmark ID and"] # [doc = " a path to a directory where the profiler can store its data."] fn start_profiling (& mut self , benchmark_id : & str , benchmark_dir : & Path) ; # [doc = " This function is called after Criterion.rs stops profiling a particular"] # [doc = " benchmark. The benchmark ID and directory are the same as in the call"] # [doc = " to [`start_profiling`](Self::start_profiling), provided for convenience."] fn stop_profiling (& mut self , benchmark_id : & str , benchmark_dir : & Path) ; }
};
}
