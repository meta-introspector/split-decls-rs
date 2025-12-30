// Generated macro for check_infer_with_mismatches (function)
macro_rules! Depcrate_consteval_testscheck_infer_with_mismatches {
() => {
// Module: crate::consteval::tests
// Provides: {"check_infer_with_mismatches"}
// Dependencies: {}
fn check_infer_with_mismatches (# [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect) { let mut actual = infer_with_mismatches (ra_fixture , true) ; actual . push ('\n') ; expect . assert_eq (& actual) ; }
};
}
