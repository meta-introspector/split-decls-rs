// Generated macro for impl_86 (impl)
macro_rules! Depcrate_remuteximpl_86 {
() => {
// Module: crate::remutex
// Provides: {"impl_86"}
// Dependencies: {}
impl < R : RawMutex , G : GetThreadId , T : ? Sized + Default > Default for ReentrantMutex < R , G , T > { # [inline] fn default () -> ReentrantMutex < R , G , T > { ReentrantMutex :: new (Default :: default ()) } }
};
}
