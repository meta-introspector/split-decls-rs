// Generated macro for sys_spinlock_irqsave_lock (function)
macro_rules! Depcrate_syscalls_spinlocksys_spinlock_irqsave_lock {
() => {
// Module: crate::syscalls::spinlock
// Provides: {"sys_spinlock_irqsave_lock"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_spinlock_irqsave_lock (lock : * mut SpinlockIrqSaveContainer < '_ >) -> i32 { if lock . is_null () { return - i32 :: from (Errno :: Inval) ; } let container = unsafe { & mut * lock } ; assert ! (container . guard . is_none () , "Called sys_spinlock_irqsave_lock when a lock is already held!") ; container . guard = Some (container . lock . lock ()) ; 0 }
};
}
