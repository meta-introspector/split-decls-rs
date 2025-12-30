// Generated macro for SpinlockContainer (struct)
macro_rules! Depcrate_syscalls_spinlockSpinlockContainer {
() => {
// Module: crate::syscalls::spinlock
// Provides: {"SpinlockContainer"}
// Dependencies: {}
pub struct SpinlockContainer < 'a > { lock : TicketMutex < () > , guard : Option < TicketMutexGuard < 'a , () > > , }
};
}
