// Generated macro for all_benchmarks (function)
macro_rules! Depcrateall_benchmarks {
() => {
// Module: crate
// Provides: {"all_benchmarks"}
// Dependencies: {}
# [doc = " Returns all benchmarks"] fn all_benchmarks () -> anyhow :: Result < Vec < Benchmark > > { let mut benchmarks = Vec :: new () ; for param in all_benchmarks_params () { add_benchmark_group (& mut benchmarks , param) ; } validate_benchmarks (& benchmarks) ? ; Ok (benchmarks) }
};
}
