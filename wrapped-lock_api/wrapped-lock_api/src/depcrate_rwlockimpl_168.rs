// Generated macro for impl_168 (impl)
macro_rules! Depcrate_rwlockimpl_168 {
() => {
// Module: crate::rwlock
// Provides: {"impl_168"}
// Dependencies: {}
# [cfg (feature = "arc_lock")] impl < R : RawRwLock , T : fmt :: Display + ? Sized > fmt :: Display for ArcRwLockReadGuard < R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
};
}
