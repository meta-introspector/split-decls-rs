// Generated macro for macro_510 (macro)
macro_rules! Depcrate_ffi_taskmacro_510 {
() => {
// Module: crate::ffi::task
// Provides: {"macro_510"}
// Dependencies: {}
ffi_fn ! { # [doc = " Free a task."] # [doc = ""] # [doc = " This should only be used if the task isn't consumed by"] # [doc = " `hyper_clientconn_handshake` or taken ownership of by"] # [doc = " `hyper_executor_push`."] fn hyper_task_free (task : * mut hyper_task) { drop (non_null ! (Box :: from_raw (task) ?= ())) ; } }
};
}
