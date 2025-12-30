// Generated macro for check_assist_by_label (function)
macro_rules! Depcrate_testscheck_assist_by_label {
() => {
// Module: crate::tests
// Provides: {"check_assist_by_label"}
// Dependencies: {}
# [track_caller] pub (crate) fn check_assist_by_label (assist : Handler , # [rust_analyzer :: rust_fixture] ra_fixture_before : & str , # [rust_analyzer :: rust_fixture] ra_fixture_after : & str , label : & str ,) { let ra_fixture_after = trim_indent (ra_fixture_after) ; check (assist , ra_fixture_before , ExpectedResult :: After (& ra_fixture_after) , Some (label)) ; }
};
}
