// Generated macro for get_process_pid (function)
macro_rules! Depcrate_processget_process_pid {
() => {
// Module: crate::process
// Provides: {"get_process_pid"}
// Dependencies: {}
fn get_process_pid (proc : HANDLE) -> u32 { unsafe { GetProcessId (proc) } }
};
}
