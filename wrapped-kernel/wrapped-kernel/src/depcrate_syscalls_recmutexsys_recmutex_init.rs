// Generated macro for sys_recmutex_init (function)
macro_rules! Depcrate_syscalls_recmutexsys_recmutex_init {
() => {
// Module: crate::syscalls::recmutex
// Provides: {"sys_recmutex_init"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_recmutex_init (recmutex : * mut * mut RecursiveMutex) -> i32 { if recmutex . is_null () { return - i32 :: from (Errno :: Inval) ; } let boxed_mutex = Box :: new (RecursiveMutex :: new ()) ; unsafe { * recmutex = Box :: into_raw (boxed_mutex) ; } 0 }
};
}
