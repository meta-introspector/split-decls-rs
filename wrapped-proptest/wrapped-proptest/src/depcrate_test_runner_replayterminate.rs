// Generated macro for terminate (function)
macro_rules! Depcrate_test_runner_replayterminate {
() => {
// Module: crate::test_runner::replay
// Provides: {"terminate"}
// Dependencies: {}
# [doc = " Append a termination mark to the given output."] pub (crate) fn terminate (mut file : impl Write) -> io :: Result < () > { write ! (file , ".") }
};
}
