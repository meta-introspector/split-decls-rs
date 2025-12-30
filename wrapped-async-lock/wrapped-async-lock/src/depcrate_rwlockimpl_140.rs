// Generated macro for impl_140 (impl)
macro_rules! Depcrate_rwlockimpl_140 {
() => {
// Module: crate::rwlock
// Provides: {"impl_140"}
// Dependencies: {}
impl < T : fmt :: Debug + ? Sized > fmt :: Debug for RwLockUpgradableReadGuard < '_ , T > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
