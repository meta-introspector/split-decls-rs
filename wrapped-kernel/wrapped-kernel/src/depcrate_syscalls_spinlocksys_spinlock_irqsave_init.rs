// Generated macro for sys_spinlock_irqsave_init (function)
macro_rules! Depcrate_syscalls_spinlocksys_spinlock_irqsave_init {
() => {
// Module: crate::syscalls::spinlock
// Provides: {"sys_spinlock_irqsave_init"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_spinlock_irqsave_init (lock : * mut * mut SpinlockIrqSaveContainer < '_ > ,) -> i32 { if lock . is_null () { return - i32 :: from (Errno :: Inval) ; } let boxed_container = Box :: new (SpinlockIrqSaveContainer { lock : InterruptTicketMutex :: new (()) , guard : None , }) ; unsafe { * lock = Box :: into_raw (boxed_container) ; } 0 }
};
}
