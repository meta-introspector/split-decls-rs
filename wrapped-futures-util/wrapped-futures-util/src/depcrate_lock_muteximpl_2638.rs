// Generated macro for impl_2638 (impl)
macro_rules! Depcrate_lock_muteximpl_2638 {
() => {
// Module: crate::lock::mutex
// Provides: {"impl_2638"}
// Dependencies: {}
impl < T : ? Sized > Deref for OwnedMutexGuard < T > { type Target = T ; fn deref (& self) -> & T { unsafe { & * self . mutex . value . get () } } }
};
}
