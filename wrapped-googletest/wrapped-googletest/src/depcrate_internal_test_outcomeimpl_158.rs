// Generated macro for impl_158 (impl)
macro_rules! Depcrate_internal_test_outcomeimpl_158 {
() => {
// Module: crate::internal::test_outcome
// Provides: {"impl_158"}
// Dependencies: {}
# [cfg (feature = "proptest")] impl From < TestAssertionFailure > for proptest :: test_runner :: TestCaseError { fn from (value : TestAssertionFailure) -> Self { proptest :: test_runner :: TestCaseError :: Fail (format ! ("{value}") . into ()) } }
};
}
