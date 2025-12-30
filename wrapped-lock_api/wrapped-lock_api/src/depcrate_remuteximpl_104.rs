// Generated macro for impl_104 (impl)
macro_rules! Depcrate_remuteximpl_104 {
() => {
// Module: crate::remutex
// Provides: {"impl_104"}
// Dependencies: {}
# [cfg (feature = "arc_lock")] impl < R : RawMutex , G : GetThreadId , T : ? Sized > Drop for ArcReentrantMutexGuard < R , G , T > { # [inline] fn drop (& mut self) { unsafe { self . remutex . raw . unlock () ; } } }
};
}
