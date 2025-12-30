// Generated macro for check_no_kw (function)
macro_rules! Depcrate_context_testscheck_no_kw {
() => {
// Module: crate::context::tests
// Provides: {"check_no_kw"}
// Dependencies: {}
pub (crate) fn check_no_kw (# [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect) { let actual = completion_list_no_kw (ra_fixture) ; expect . assert_eq (& actual) }
};
}
