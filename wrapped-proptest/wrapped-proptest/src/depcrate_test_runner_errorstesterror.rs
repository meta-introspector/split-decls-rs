// Generated macro for TestError (enum)
macro_rules! Depcrate_test_runner_errorsTestError {
() => {
// Module: crate::test_runner::errors
// Provides: {"TestError"}
// Dependencies: {}
# [doc = " A failure state from running test cases for a single test."] # [derive (Debug , Clone , PartialEq , Eq)] pub enum TestError < T > { # [doc = " The test was aborted for the given reason, for example, due to too many"] # [doc = " inputs having been rejected."] Abort (Reason) , # [doc = " A failing test case was found. The string indicates where and/or why"] # [doc = " the test failed. The `T` is the minimal input found to reproduce the"] # [doc = " failure."] Fail (Reason , T) , }
};
}
