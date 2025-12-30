// Generated macro for impl_190 (impl)
macro_rules! Depcrate_rwlockimpl_190 {
() => {
// Module: crate::rwlock
// Provides: {"impl_190"}
// Dependencies: {}
# [cfg (feature = "arc_lock")] impl < R : RawRwLock , T : fmt :: Display + ? Sized > fmt :: Display for ArcRwLockWriteGuard < R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
};
}
