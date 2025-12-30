// Generated macro for check (function)
macro_rules! Depcrate_context_testscheck {
() => {
// Module: crate::context::tests
// Provides: {"check"}
// Dependencies: {}
pub (crate) fn check (# [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect) { let actual = completion_list (ra_fixture) ; expect . assert_eq (& actual) ; }
};
}
