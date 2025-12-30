// Generated macro for impl_57 (impl)
macro_rules! Depcrate_muteximpl_57 {
() => {
// Module: crate::mutex
// Provides: {"impl_57"}
// Dependencies: {}
impl < 'a , R : RawMutex + 'a , T : ? Sized + 'a > DerefMut for MappedMutexGuard < 'a , R , T > { # [inline] fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . data } } }
};
}
