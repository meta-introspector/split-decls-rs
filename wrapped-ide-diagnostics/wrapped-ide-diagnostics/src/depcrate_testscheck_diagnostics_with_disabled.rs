// Generated macro for check_diagnostics_with_disabled (function)
macro_rules! Depcrate_testscheck_diagnostics_with_disabled {
() => {
// Module: crate::tests
// Provides: {"check_diagnostics_with_disabled"}
// Dependencies: {}
# [track_caller] pub (crate) fn check_diagnostics_with_disabled (# [rust_analyzer :: rust_fixture] ra_fixture : & str , disabled : & [& str] ,) { let mut config = DiagnosticsConfig :: test_sample () ; config . disabled . extend (disabled . iter () . map (| & s | s . to_owned ())) ; check_diagnostics_with_config (config , ra_fixture) }
};
}
