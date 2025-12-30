// Generated macro for check_if_tidy_is_installed (function)
macro_rules! Depcrate_core_build_steps_testcheck_if_tidy_is_installed {
() => {
// Module: crate::core::build_steps::test
// Provides: {"check_if_tidy_is_installed"}
// Dependencies: {}
fn check_if_tidy_is_installed (builder : & Builder < '_ >) -> bool { command ("tidy") . allow_failure () . arg ("--version") . run_capture_stdout (builder) . is_success () }
};
}
