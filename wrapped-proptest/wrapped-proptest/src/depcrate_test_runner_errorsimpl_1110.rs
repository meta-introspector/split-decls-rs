// Generated macro for impl_1110 (impl)
macro_rules! Depcrate_test_runner_errorsimpl_1110 {
() => {
// Module: crate::test_runner::errors
// Provides: {"impl_1110"}
// Dependencies: {}
# [cfg (feature = "std")] impl < E : :: std :: error :: Error > From < E > for TestCaseError { fn from (cause : E) -> Self { TestCaseError :: fail (cause . to_string ()) } }
};
}
