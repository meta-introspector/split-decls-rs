// Generated macro for impl_2655 (impl)
macro_rules! Depcrate_lock_muteximpl_2655 {
() => {
// Module: crate::lock::mutex
// Provides: {"impl_2655"}
// Dependencies: {}
impl < T : ? Sized , U : ? Sized > Deref for MappedMutexGuard < '_ , T , U > { type Target = U ; fn deref (& self) -> & U { unsafe { & * self . value } } }
};
}
