// Generated macro for sys_recmutex_unlock (function)
macro_rules! Depcrate_syscalls_recmutexsys_recmutex_unlock {
() => {
// Module: crate::syscalls::recmutex
// Provides: {"sys_recmutex_unlock"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_recmutex_unlock (recmutex : * mut RecursiveMutex) -> i32 { if recmutex . is_null () { return - i32 :: from (Errno :: Inval) ; } let mutex = unsafe { & * recmutex } ; mutex . release () ; 0 }
};
}
