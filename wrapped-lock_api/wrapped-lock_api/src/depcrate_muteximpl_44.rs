// Generated macro for impl_44 (impl)
macro_rules! Depcrate_muteximpl_44 {
() => {
// Module: crate::mutex
// Provides: {"impl_44"}
// Dependencies: {}
# [cfg (feature = "arc_lock")] unsafe impl < R : RawMutex + Send + Sync , T : Send + ? Sized > Send for ArcMutexGuard < R , T > where R :: GuardMarker : Send { }
};
}
