// Generated macro for sys_spinlock_init (function)
macro_rules! Depcrate_syscalls_spinlocksys_spinlock_init {
() => {
// Module: crate::syscalls::spinlock
// Provides: {"sys_spinlock_init"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_spinlock_init (lock : * mut * mut SpinlockContainer < '_ >) -> i32 { if lock . is_null () { return - i32 :: from (Errno :: Inval) ; } let boxed_container = Box :: new (SpinlockContainer { lock : TicketMutex :: new (()) , guard : None , }) ; unsafe { * lock = Box :: into_raw (boxed_container) ; } 0 }
};
}
