// Generated macro for impl_87 (impl)
macro_rules! Depcrate_remuteximpl_87 {
() => {
// Module: crate::remutex
// Provides: {"impl_87"}
// Dependencies: {}
impl < R : RawMutex , G : GetThreadId , T > From < T > for ReentrantMutex < R , G , T > { # [inline] fn from (t : T) -> ReentrantMutex < R , G , T > { ReentrantMutex :: new (t) } }
};
}
