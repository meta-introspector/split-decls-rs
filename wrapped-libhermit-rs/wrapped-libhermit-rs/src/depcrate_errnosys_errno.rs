// Generated macro for sys_errno (function)
macro_rules! Depcrate_errnosys_errno {
() => {
// Module: crate::errno
// Provides: {"sys_errno"}
// Dependencies: {}
# [doc = " Get the error number from the thread local storage"] # [doc = ""] # [doc = " Soft-deprecated in favor of using `sys_errno_location`."] # [cfg (not (feature = "nostd"))] # [unsafe (no_mangle)] pub extern "C" fn sys_errno () -> i32 { cfg_if :: cfg_if ! { if # [cfg (any (feature = "common-os" , target_arch = "riscv64"))] { 0 } else { unsafe { * sys_errno_location () } } } }
};
}
