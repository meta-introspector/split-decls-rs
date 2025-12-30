// Generated macro for benchmark_function (function)
macro_rules! Depcrate_measure_perf_counter_non_linuxbenchmark_function {
() => {
// Module: crate::measure::perf_counter::non_linux
// Provides: {"benchmark_function"}
// Dependencies: {}
pub fn benchmark_function < F : FnOnce () -> R , R > (_func : F) -> anyhow :: Result < BenchmarkStats > { panic ! ("Runtime benchmarking is only supported on Linux") ; }
};
}
