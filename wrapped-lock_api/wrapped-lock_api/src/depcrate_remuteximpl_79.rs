// Generated macro for impl_79 (impl)
macro_rules! Depcrate_remuteximpl_79 {
() => {
// Module: crate::remutex
// Provides: {"impl_79"}
// Dependencies: {}
unsafe impl < R : RawMutex + Send , G : GetThreadId + Send , T : ? Sized + Send > Send for ReentrantMutex < R , G , T > { }
};
}
