// Generated macro for impl_45 (impl)
macro_rules! Depcrate_muteximpl_45 {
() => {
// Module: crate::mutex
// Provides: {"impl_45"}
// Dependencies: {}
# [cfg (feature = "arc_lock")] unsafe impl < R : RawMutex + Sync , T : Sync + ? Sized > Sync for ArcMutexGuard < R , T > where R :: GuardMarker : Sync { }
};
}
