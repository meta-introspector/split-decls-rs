// Generated macro for is_process_alive (function)
macro_rules! Depcrate_processis_process_alive {
() => {
// Module: crate::process
// Provides: {"is_process_alive"}
// Dependencies: {}
fn is_process_alive (proc : HANDLE) -> bool { unsafe { WaitForSingleObject (proc , 0) == WAIT_TIMEOUT } }
};
}
