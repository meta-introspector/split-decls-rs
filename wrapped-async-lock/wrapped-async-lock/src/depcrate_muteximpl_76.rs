// Generated macro for impl_76 (impl)
macro_rules! Depcrate_muteximpl_76 {
() => {
// Module: crate::mutex
// Provides: {"impl_76"}
// Dependencies: {}
impl < T : ? Sized > DerefMut for MutexGuardArc < T > { fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . 0 . data . get () } } }
};
}
