// Generated macro for impl_75 (impl)
macro_rules! Depcrate_muteximpl_75 {
() => {
// Module: crate::mutex
// Provides: {"impl_75"}
// Dependencies: {}
impl < T : ? Sized > Deref for MutexGuardArc < T > { type Target = T ; fn deref (& self) -> & T { unsafe { & * self . 0 . data . get () } } }
};
}
