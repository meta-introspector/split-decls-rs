// Generated macro for bigint_fuzz_iteration_count (function)
macro_rules! Depcrate_run_cfgbigint_fuzz_iteration_count {
() => {
// Module: crate::run_cfg
// Provides: {"bigint_fuzz_iteration_count"}
// Dependencies: {}
# [doc = " The number of iterations to run for `u256` fuzz tests."] pub fn bigint_fuzz_iteration_count () -> u64 { if ! cfg ! (optimizations_enabled) { return 1000 ; } if slow_platform () { 100_000 } else { 5_000_000 } }
};
}
