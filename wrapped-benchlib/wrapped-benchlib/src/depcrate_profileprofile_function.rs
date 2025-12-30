// Generated macro for profile_function (function)
macro_rules! Depcrate_profileprofile_function {
() => {
// Module: crate::profile
// Provides: {"profile_function"}
// Dependencies: {}
pub fn profile_function < F : Fn () -> Bench , R , Bench : FnOnce () -> R > (benchmark_constructor : & F) { let func = benchmark_constructor () ; # [cfg (feature = "precise-cachegrind")] { crabgrind :: cachegrind :: start_instrumentation () ; } func () ; # [cfg (feature = "precise-cachegrind")] { crabgrind :: cachegrind :: stop_instrumentation () ; } }
};
}
