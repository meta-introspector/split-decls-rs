// Generated macro for sys_recmutex_lock (function)
macro_rules! Depcrate_syscalls_recmutexsys_recmutex_lock {
() => {
// Module: crate::syscalls::recmutex
// Provides: {"sys_recmutex_lock"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_recmutex_lock (recmutex : * mut RecursiveMutex) -> i32 { if recmutex . is_null () { return - i32 :: from (Errno :: Inval) ; } let mutex = unsafe { & * recmutex } ; mutex . acquire () ; 0 }
};
}
