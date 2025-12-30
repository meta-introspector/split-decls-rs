// Generated macro for await_child (function)
macro_rules! Depcrate_test_runner_runnerawait_child {
() => {
// Module: crate::test_runner::runner
// Provides: {"await_child"}
// Dependencies: {}
# [cfg (all (feature = "fork" , feature = "timeout"))] fn await_child (child : & mut rusty_fork :: ChildWrapper , forkfile : & mut tempfile :: NamedTempFile , timeout : u32 ,) -> (Option < TestCaseError > , Option < u64 >) { use std :: time :: Duration ; if 0 == timeout { return await_child_without_timeout (child) ; } let mut last_forkfile_len = forkfile . as_file () . metadata () . map (| md | md . len ()) . unwrap_or (0) ; loop { if let Some (status) = child . wait_timeout (Duration :: from_millis (timeout . into ())) . expect ("Failed to wait for child process") { if status . success () { return (None , None) ; } else { return (Some (TestCaseError :: fail (format ! ("Child process exited with {}" , status))) , None ,) ; } } let current_len = forkfile . as_file () . metadata () . map (| md | md . len ()) . unwrap_or (0) ; if current_len <= last_forkfile_len { return (Some (TestCaseError :: fail (format ! ("Timed out waiting for child process"))) , Some (current_len) ,) ; } else { last_forkfile_len = current_len ; } } }
};
}
