// Generated macro for check_infer (function)
macro_rules! Depcrate_consteval_testscheck_infer {
() => {
// Module: crate::consteval::tests
// Provides: {"check_infer"}
// Dependencies: {}
fn check_infer (# [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect) { let mut actual = infer (ra_fixture) ; actual . push ('\n') ; expect . assert_eq (& actual) ; }
};
}
