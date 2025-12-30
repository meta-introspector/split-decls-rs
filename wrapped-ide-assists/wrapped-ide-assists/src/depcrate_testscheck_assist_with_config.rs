// Generated macro for check_assist_with_config (function)
macro_rules! Depcrate_testscheck_assist_with_config {
() => {
// Module: crate::tests
// Provides: {"check_assist_with_config"}
// Dependencies: {}
# [track_caller] pub (crate) fn check_assist_with_config (assist : Handler , config : AssistConfig , # [rust_analyzer :: rust_fixture] ra_fixture_before : & str , # [rust_analyzer :: rust_fixture] ra_fixture_after : & str ,) { let ra_fixture_after = trim_indent (ra_fixture_after) ; check_with_config (config , assist , ra_fixture_before , ExpectedResult :: After (& ra_fixture_after) , None ,) ; }
};
}
