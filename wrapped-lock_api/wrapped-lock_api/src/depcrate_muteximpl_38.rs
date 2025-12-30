// Generated macro for impl_38 (impl)
macro_rules! Depcrate_muteximpl_38 {
() => {
// Module: crate::mutex
// Provides: {"impl_38"}
// Dependencies: {}
impl < 'a , R : RawMutex + 'a , T : ? Sized + 'a > DerefMut for MutexGuard < 'a , R , T > { # [inline] fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . mutex . data . get () } } }
};
}
