// Generated macro for await_child_without_timeout (function)
macro_rules! Depcrate_test_runner_runnerawait_child_without_timeout {
() => {
// Module: crate::test_runner::runner
// Provides: {"await_child_without_timeout"}
// Dependencies: {}
# [cfg (feature = "fork")] fn await_child_without_timeout (child : & mut rusty_fork :: ChildWrapper ,) -> (Option < TestCaseError > , Option < u64 >) { let status = child . wait () . expect ("Failed to wait for child process") ; if status . success () { (None , None) } else { (Some (TestCaseError :: fail (format ! ("Child process exited with {}" , status))) , None ,) } }
};
}
