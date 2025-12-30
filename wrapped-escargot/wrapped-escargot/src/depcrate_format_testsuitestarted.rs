// Generated macro for SuiteStarted (struct)
macro_rules! Depcrate_format_testSuiteStarted {
() => {
// Module: crate::format::test
// Provides: {"SuiteStarted"}
// Dependencies: {}
# [doc = " Suite-started event."] # [derive (Serialize , Deserialize , Clone , Debug , Eq , PartialEq)] # [non_exhaustive] pub struct SuiteStarted { # [doc = " Number of test cases in the suite."] pub test_count : usize , }
};
}
