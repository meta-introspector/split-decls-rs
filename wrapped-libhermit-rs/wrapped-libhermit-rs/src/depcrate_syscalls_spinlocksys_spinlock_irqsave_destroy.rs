// Generated macro for sys_spinlock_irqsave_destroy (function)
macro_rules! Depcrate_syscalls_spinlocksys_spinlock_irqsave_destroy {
() => {
// Module: crate::syscalls::spinlock
// Provides: {"sys_spinlock_irqsave_destroy"}
// Dependencies: {}
# [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_spinlock_irqsave_destroy (lock : * mut SpinlockIrqSaveContainer < '_ > ,) -> i32 { if lock . is_null () { return - i32 :: from (Errno :: Inval) ; } unsafe { drop (Box :: from_raw (lock)) ; } 0 }
};
}
