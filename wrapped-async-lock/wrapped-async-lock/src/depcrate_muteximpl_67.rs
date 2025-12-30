// Generated macro for impl_67 (impl)
macro_rules! Depcrate_muteximpl_67 {
() => {
// Module: crate::mutex
// Provides: {"impl_67"}
// Dependencies: {}
impl < T : ? Sized > DerefMut for MutexGuard < '_ , T > { fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . 0 . data . get () } } }
};
}
