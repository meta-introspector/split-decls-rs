// Generated macro for impl_103 (impl)
macro_rules! Depcrate_remuteximpl_103 {
() => {
// Module: crate::remutex
// Provides: {"impl_103"}
// Dependencies: {}
# [cfg (feature = "arc_lock")] impl < R : RawMutex , G : GetThreadId , T : ? Sized > Deref for ArcReentrantMutexGuard < R , G , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . remutex . data . get () } } }
};
}
