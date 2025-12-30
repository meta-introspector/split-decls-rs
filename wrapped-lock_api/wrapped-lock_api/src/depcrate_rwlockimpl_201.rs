// Generated macro for impl_201 (impl)
macro_rules! Depcrate_rwlockimpl_201 {
() => {
// Module: crate::rwlock
// Provides: {"impl_201"}
// Dependencies: {}
impl < 'a , R : RawRwLockUpgrade + 'a , T : fmt :: Display + ? Sized + 'a > fmt :: Display for RwLockUpgradableReadGuard < 'a , R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* * self) . fmt (f) } }
};
}
