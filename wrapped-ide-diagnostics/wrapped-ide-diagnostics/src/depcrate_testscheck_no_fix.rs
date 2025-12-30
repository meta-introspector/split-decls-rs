// Generated macro for check_no_fix (function)
macro_rules! Depcrate_testscheck_no_fix {
() => {
// Module: crate::tests
// Provides: {"check_no_fix"}
// Dependencies: {}
# [doc = " Checks that there's a diagnostic *without* fix at `$0`."] pub (crate) fn check_no_fix (# [rust_analyzer :: rust_fixture] ra_fixture : & str) { let (db , file_position) = RootDatabase :: with_position (ra_fixture) ; let diagnostic = hir :: attach_db (& db , | | { super :: full_diagnostics (& db , & DiagnosticsConfig :: test_sample () , & AssistResolveStrategy :: All , file_position . file_id . file_id (& db) ,) }) . pop () . unwrap () ; assert ! (diagnostic . fixes . is_none () , "got a fix when none was expected: {diagnostic:?}") ; }
};
}
