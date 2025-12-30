// Generated macro for sys_get_errno (function)
macro_rules! Depcrate_errnosys_get_errno {
() => {
// Module: crate::errno
// Provides: {"sys_get_errno"}
// Dependencies: {}
# [doc = " Get the error number from the thread local storage"] # [doc = ""] # [doc = " Soft-deprecated in favor of using `sys_errno_location`."] # [cfg (not (feature = "nostd"))] # [unsafe (no_mangle)] pub extern "C" fn sys_get_errno () -> i32 { sys_errno () }
};
}
