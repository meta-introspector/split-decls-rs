// Generated macro for hyper_task_return_type (enum)
macro_rules! Depcrate_ffi_taskhyper_task_return_type {
() => {
// Module: crate::ffi::task
// Provides: {"hyper_task_return_type"}
// Dependencies: {}
# [doc = " A descriptor for what type a `hyper_task` value is."] # [repr (C)] pub enum hyper_task_return_type { # [doc = " The value of this task is null (does not imply an error)."] HYPER_TASK_EMPTY , # [doc = " The value of this task is `hyper_error *`."] HYPER_TASK_ERROR , # [doc = " The value of this task is `hyper_clientconn *`."] HYPER_TASK_CLIENTCONN , # [doc = " The value of this task is `hyper_response *`."] HYPER_TASK_RESPONSE , # [doc = " The value of this task is `hyper_buf *`."] HYPER_TASK_BUF , }
};
}
