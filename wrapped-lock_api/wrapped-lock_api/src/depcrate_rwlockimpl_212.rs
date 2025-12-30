// Generated macro for impl_212 (impl)
macro_rules! Depcrate_rwlockimpl_212 {
() => {
// Module: crate::rwlock
// Provides: {"impl_212"}
// Dependencies: {}
# [cfg (feature = "arc_lock")] impl < R : RawRwLockUpgrade , T : fmt :: Display + ? Sized > fmt :: Display for ArcRwLockUpgradableReadGuard < R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
};
}
