// Generated macro for append (function)
macro_rules! Depcrate_test_runner_replayappend {
() => {
// Module: crate::test_runner::replay
// Provides: {"append"}
// Dependencies: {}
# [doc = " Append the given step to the given output."] pub (crate) fn append (mut file : impl Write , step : & TestCaseResult ,) -> io :: Result < () > { write ! (file , "{}" , step_to_char (step)) }
};
}
