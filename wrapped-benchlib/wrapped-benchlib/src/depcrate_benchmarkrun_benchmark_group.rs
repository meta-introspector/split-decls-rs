// Generated macro for run_benchmark_group (function)
macro_rules! Depcrate_benchmarkrun_benchmark_group {
() => {
// Module: crate::benchmark
// Provides: {"run_benchmark_group"}
// Dependencies: {}
# [doc = " Create and run a new benchmark group. Use the closure argument to register"] # [doc = " the individual benchmarks."] pub fn run_benchmark_group < 'a , F > (register : F) where F : FnOnce (& mut BenchmarkGroup < 'a >) , { env_logger :: init () ; let mut group : BenchmarkGroup < 'a > = BenchmarkGroup :: default () ; register (& mut group) ; group . run () . expect ("Benchmark group execution has failed") ; }
};
}
