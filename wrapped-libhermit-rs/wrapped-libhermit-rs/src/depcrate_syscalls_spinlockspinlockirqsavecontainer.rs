// Generated macro for SpinlockIrqSaveContainer (struct)
macro_rules! Depcrate_syscalls_spinlockSpinlockIrqSaveContainer {
() => {
// Module: crate::syscalls::spinlock
// Provides: {"SpinlockIrqSaveContainer"}
// Dependencies: {}
pub struct SpinlockIrqSaveContainer < 'a > { lock : InterruptTicketMutex < () > , guard : Option < InterruptTicketMutexGuard < 'a , () > > , }
};
}
