// Generated macro for impl_2639 (impl)
macro_rules! Depcrate_lock_muteximpl_2639 {
() => {
// Module: crate::lock::mutex
// Provides: {"impl_2639"}
// Dependencies: {}
impl < T : ? Sized > DerefMut for OwnedMutexGuard < T > { fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . mutex . value . get () } } }
};
}
