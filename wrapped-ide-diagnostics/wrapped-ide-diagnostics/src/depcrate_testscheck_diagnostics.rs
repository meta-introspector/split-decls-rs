// Generated macro for check_diagnostics (function)
macro_rules! Depcrate_testscheck_diagnostics {
() => {
// Module: crate::tests
// Provides: {"check_diagnostics"}
// Dependencies: {}
# [track_caller] pub (crate) fn check_diagnostics (# [rust_analyzer :: rust_fixture] ra_fixture : & str) { let mut config = DiagnosticsConfig :: test_sample () ; config . disabled . insert ("inactive-code" . to_owned ()) ; check_diagnostics_with_config (config , ra_fixture) }
};
}
