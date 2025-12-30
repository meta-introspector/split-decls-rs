// Generated macro for check_assist_import_one (function)
macro_rules! Depcrate_testscheck_assist_import_one {
() => {
// Module: crate::tests
// Provides: {"check_assist_import_one"}
// Dependencies: {}
# [track_caller] pub (crate) fn check_assist_import_one (assist : Handler , # [rust_analyzer :: rust_fixture] ra_fixture_before : & str , # [rust_analyzer :: rust_fixture] ra_fixture_after : & str ,) { let ra_fixture_after = trim_indent (ra_fixture_after) ; check_with_config (TEST_CONFIG_IMPORT_ONE , assist , ra_fixture_before , ExpectedResult :: After (& ra_fixture_after) , None ,) ; }
};
}
