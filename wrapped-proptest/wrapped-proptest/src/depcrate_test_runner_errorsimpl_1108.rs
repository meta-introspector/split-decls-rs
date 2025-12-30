// Generated macro for impl_1108 (impl)
macro_rules! Depcrate_test_runner_errorsimpl_1108 {
() => {
// Module: crate::test_runner::errors
// Provides: {"impl_1108"}
// Dependencies: {}
impl TestCaseError { # [doc = " Rejects the generated test input as invalid for this test case. This"] # [doc = " does not count as a test failure (nor a success); rather, it simply"] # [doc = " signals to generate a new input and try again."] # [doc = ""] # [doc = " The string gives the location and context of the rejection, and"] # [doc = " should be suitable for formatting like `Foo did X at {whence}`."] pub fn reject (reason : impl Into < Reason >) -> Self { TestCaseError :: Reject (reason . into ()) } # [doc = " The code under test failed the test."] # [doc = ""] # [doc = " The string should indicate the location of the failure, but may"] # [doc = " generally be any string."] pub fn fail (reason : impl Into < Reason >) -> Self { TestCaseError :: Fail (reason . into ()) } }
};
}
