// Generated macro for impl_80 (impl)
macro_rules! Depcrate_remuteximpl_80 {
() => {
// Module: crate::remutex
// Provides: {"impl_80"}
// Dependencies: {}
unsafe impl < R : RawMutex + Sync , G : GetThreadId + Sync , T : ? Sized + Send > Sync for ReentrantMutex < R , G , T > { }
};
}
