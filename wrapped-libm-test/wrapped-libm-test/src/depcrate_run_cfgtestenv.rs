// Generated macro for TestEnv (struct)
macro_rules! Depcrate_run_cfgTestEnv {
() => {
// Module: crate::run_cfg
// Provides: {"TestEnv"}
// Dependencies: {}
# [doc = " Information about the function to be tested."] # [derive (Debug)] struct TestEnv { # [doc = " Tests should be reduced because the platform is slow. E.g. 32-bit or emulated."] slow_platform : bool , # [doc = " The float cannot be tested exhaustively, `f64` or `f128`."] large_float_ty : bool , # [doc = " Env indicates that an extensive test should be run."] should_run_extensive : bool , # [doc = " Multiprecision tests will be run."] mp_tests_enabled : bool , # [doc = " The number of inputs to the function."] input_count : usize , }
};
}
