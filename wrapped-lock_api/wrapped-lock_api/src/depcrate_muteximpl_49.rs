// Generated macro for impl_49 (impl)
macro_rules! Depcrate_muteximpl_49 {
() => {
// Module: crate::mutex
// Provides: {"impl_49"}
// Dependencies: {}
# [cfg (feature = "arc_lock")] impl < R : RawMutex , T : ? Sized > DerefMut for ArcMutexGuard < R , T > { # [inline] fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . mutex . data . get () } } }
};
}
