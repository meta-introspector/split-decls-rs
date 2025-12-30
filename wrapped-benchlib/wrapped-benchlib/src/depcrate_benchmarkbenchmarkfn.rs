// Generated macro for BenchmarkFn (type)
macro_rules! Depcrate_benchmarkBenchmarkFn {
() => {
// Module: crate::benchmark
// Provides: {"BenchmarkFn"}
// Dependencies: {}
# [doc = " Type-erased function that executes a single benchmark and measures counter and wall-time"] # [doc = " metrics."] type BenchmarkFn < 'a > = Box < dyn Fn () -> anyhow :: Result < BenchmarkStats > + 'a > ;
};
}
