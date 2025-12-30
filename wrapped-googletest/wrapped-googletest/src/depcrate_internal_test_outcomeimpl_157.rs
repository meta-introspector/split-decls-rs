// Generated macro for impl_157 (impl)
macro_rules! Depcrate_internal_test_outcomeimpl_157 {
() => {
// Module: crate::internal::test_outcome
// Provides: {"impl_157"}
// Dependencies: {}
impl < T : std :: error :: Error > From < T > for TestAssertionFailure { # [track_caller] fn from (value : T) -> Self { TestAssertionFailure :: create (format ! ("{value}")) } }
};
}
