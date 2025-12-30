// Generated macro for macro_514 (macro)
macro_rules! Depcrate_ffi_taskmacro_514 {
() => {
// Module: crate::ffi::task
// Provides: {"macro_514"}
// Dependencies: {}
ffi_fn ! { # [doc = " Retrieve the userdata that has been set via `hyper_task_set_userdata`."] fn hyper_task_userdata (task : * mut hyper_task) -> * mut c_void { non_null ! (&* task ?= ptr :: null_mut ()) . userdata . 0 } ?= ptr :: null_mut () }
};
}
