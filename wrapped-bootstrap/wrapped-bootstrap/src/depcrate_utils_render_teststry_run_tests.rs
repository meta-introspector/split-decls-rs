// Generated macro for try_run_tests (function)
macro_rules! Depcrate_utils_render_teststry_run_tests {
() => {
// Module: crate::utils::render_tests
// Provides: {"try_run_tests"}
// Dependencies: {}
pub (crate) fn try_run_tests (builder : & Builder < '_ > , cmd : & mut BootstrapCommand , stream : bool ,) -> bool { if run_tests (builder , cmd , stream) { return true ; } if builder . fail_fast { crate :: exit ! (1) ; } builder . config . exec_ctx () . add_to_delay_failure (format ! ("{cmd:?}")) ; false }
};
}
