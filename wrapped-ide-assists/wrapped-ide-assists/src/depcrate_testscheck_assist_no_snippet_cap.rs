// Generated macro for check_assist_no_snippet_cap (function)
macro_rules! Depcrate_testscheck_assist_no_snippet_cap {
() => {
// Module: crate::tests
// Provides: {"check_assist_no_snippet_cap"}
// Dependencies: {}
# [track_caller] pub (crate) fn check_assist_no_snippet_cap (assist : Handler , # [rust_analyzer :: rust_fixture] ra_fixture_before : & str , # [rust_analyzer :: rust_fixture] ra_fixture_after : & str ,) { let ra_fixture_after = trim_indent (ra_fixture_after) ; check_with_config (TEST_CONFIG_NO_SNIPPET_CAP , assist , ra_fixture_before , ExpectedResult :: After (& ra_fixture_after) , None ,) ; }
};
}
