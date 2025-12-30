// Generated macro for impl_2649 (impl)
macro_rules! Depcrate_lock_muteximpl_2649 {
() => {
// Module: crate::lock::mutex
// Provides: {"impl_2649"}
// Dependencies: {}
impl < T : ? Sized > Deref for MutexGuard < '_ , T > { type Target = T ; fn deref (& self) -> & T { unsafe { & * self . mutex . value . get () } } }
};
}
