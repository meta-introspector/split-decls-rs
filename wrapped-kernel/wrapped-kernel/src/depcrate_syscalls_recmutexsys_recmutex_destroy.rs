// Generated macro for sys_recmutex_destroy (function)
macro_rules! Depcrate_syscalls_recmutexsys_recmutex_destroy {
() => {
// Module: crate::syscalls::recmutex
// Provides: {"sys_recmutex_destroy"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_recmutex_destroy (recmutex : * mut RecursiveMutex) -> i32 { if recmutex . is_null () { return - i32 :: from (Errno :: Inval) ; } unsafe { drop (Box :: from_raw (recmutex)) ; } 0 }
};
}
