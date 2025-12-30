// Generated macro for check_with_private_editable (function)
macro_rules! Depcrate_testscheck_with_private_editable {
() => {
// Module: crate::tests
// Provides: {"check_with_private_editable"}
// Dependencies: {}
pub (crate) fn check_with_private_editable (# [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect ,) { let actual = completion_list_no_kw_with_private_editable (ra_fixture) ; expect . assert_eq (& actual) ; }
};
}
