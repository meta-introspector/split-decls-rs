// Generated macro for impl_66 (impl)
macro_rules! Depcrate_muteximpl_66 {
() => {
// Module: crate::mutex
// Provides: {"impl_66"}
// Dependencies: {}
impl < T : ? Sized > Deref for MutexGuard < '_ , T > { type Target = T ; fn deref (& self) -> & T { unsafe { & * self . 0 . data . get () } } }
};
}
