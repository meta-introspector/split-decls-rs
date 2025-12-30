// Generated macro for impl_200 (impl)
macro_rules! Depcrate_rwlockimpl_200 {
() => {
// Module: crate::rwlock
// Provides: {"impl_200"}
// Dependencies: {}
impl < 'a , R : RawRwLockUpgrade + 'a , T : fmt :: Debug + ? Sized + 'a > fmt :: Debug for RwLockUpgradableReadGuard < 'a , R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
