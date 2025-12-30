// Generated macro for check_assist (function)
macro_rules! Depcrate_testscheck_assist {
() => {
// Module: crate::tests
// Provides: {"check_assist"}
// Dependencies: {}
# [track_caller] pub (crate) fn check_assist (assist : Handler , # [rust_analyzer :: rust_fixture] ra_fixture_before : & str , # [rust_analyzer :: rust_fixture] ra_fixture_after : & str ,) { let ra_fixture_after = trim_indent (ra_fixture_after) ; check (assist , ra_fixture_before , ExpectedResult :: After (& ra_fixture_after) , None) ; }
};
}
