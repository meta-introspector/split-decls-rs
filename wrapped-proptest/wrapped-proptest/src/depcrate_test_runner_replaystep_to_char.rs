// Generated macro for step_to_char (function)
macro_rules! Depcrate_test_runner_replaystep_to_char {
() => {
// Module: crate::test_runner::replay
// Provides: {"step_to_char"}
// Dependencies: {}
fn step_to_char (step : & TestCaseResult) -> char { match * step { Ok (_) => '+' , Err (TestCaseError :: Reject (_)) => '!' , Err (TestCaseError :: Fail (_)) => '-' , } }
};
}
