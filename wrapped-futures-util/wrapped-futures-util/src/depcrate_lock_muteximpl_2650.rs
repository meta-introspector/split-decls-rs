// Generated macro for impl_2650 (impl)
macro_rules! Depcrate_lock_muteximpl_2650 {
() => {
// Module: crate::lock::mutex
// Provides: {"impl_2650"}
// Dependencies: {}
impl < T : ? Sized > DerefMut for MutexGuard < '_ , T > { fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . mutex . value . get () } } }
};
}
