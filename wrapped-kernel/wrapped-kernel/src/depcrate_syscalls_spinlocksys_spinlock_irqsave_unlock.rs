// Generated macro for sys_spinlock_irqsave_unlock (function)
macro_rules! Depcrate_syscalls_spinlocksys_spinlock_irqsave_unlock {
() => {
// Module: crate::syscalls::spinlock
// Provides: {"sys_spinlock_irqsave_unlock"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_spinlock_irqsave_unlock (lock : * mut SpinlockIrqSaveContainer < '_ > ,) -> i32 { if lock . is_null () { return - i32 :: from (Errno :: Inval) ; } let container = unsafe { & mut * lock } ; assert ! (container . guard . is_some () , "Called sys_spinlock_irqsave_unlock when no lock is currently held!") ; container . guard = None ; 0 }
};
}
