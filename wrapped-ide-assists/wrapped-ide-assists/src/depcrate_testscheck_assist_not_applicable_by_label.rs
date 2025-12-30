// Generated macro for check_assist_not_applicable_by_label (function)
macro_rules! Depcrate_testscheck_assist_not_applicable_by_label {
() => {
// Module: crate::tests
// Provides: {"check_assist_not_applicable_by_label"}
// Dependencies: {}
# [track_caller] pub (crate) fn check_assist_not_applicable_by_label (assist : Handler , # [rust_analyzer :: rust_fixture] ra_fixture : & str , label : & str ,) { check (assist , ra_fixture , ExpectedResult :: NotApplicable , Some (label)) ; }
};
}
