// Generated macro for impl_48 (impl)
macro_rules! Depcrate_muteximpl_48 {
() => {
// Module: crate::mutex
// Provides: {"impl_48"}
// Dependencies: {}
# [cfg (feature = "arc_lock")] impl < R : RawMutex , T : ? Sized > Deref for ArcMutexGuard < R , T > { type Target = T ; # [inline] fn deref (& self) -> & T { unsafe { & * self . mutex . data . get () } } }
};
}
