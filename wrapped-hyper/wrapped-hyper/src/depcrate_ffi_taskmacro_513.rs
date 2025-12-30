// Generated macro for macro_513 (macro)
macro_rules! Depcrate_ffi_taskmacro_513 {
() => {
// Module: crate::ffi::task
// Provides: {"macro_513"}
// Dependencies: {}
ffi_fn ! { # [doc = " Set a user data pointer to be associated with this task."] # [doc = ""] # [doc = " This value will be passed to task callbacks, and can be checked later"] # [doc = " with `hyper_task_userdata`."] # [doc = ""] # [doc = " This is useful for telling apart tasks for different requests that are"] # [doc = " running on the same executor."] fn hyper_task_set_userdata (task : * mut hyper_task , userdata : * mut c_void) { if task . is_null () { return ; } unsafe { (* task) . userdata = UserDataPointer (userdata) } ; } }
};
}
