// Generated macro for impl_50 (impl)
macro_rules! Depcrate_muteximpl_50 {
() => {
// Module: crate::mutex
// Provides: {"impl_50"}
// Dependencies: {}
# [cfg (feature = "arc_lock")] impl < R : RawMutex , T : ? Sized > Drop for ArcMutexGuard < R , T > { # [inline] fn drop (& mut self) { unsafe { self . mutex . raw . unlock () ; } } }
};
}
