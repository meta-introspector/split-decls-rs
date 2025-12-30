// Generated macro for macro_512 (macro)
macro_rules! Depcrate_ffi_taskmacro_512 {
() => {
// Module: crate::ffi::task
// Provides: {"macro_512"}
// Dependencies: {}
ffi_fn ! { # [doc = " Query the return type of this task."] fn hyper_task_type (task : * mut hyper_task) -> hyper_task_return_type { non_null ! (&* task ?= hyper_task_return_type :: HYPER_TASK_EMPTY) . output_type () } }
};
}
