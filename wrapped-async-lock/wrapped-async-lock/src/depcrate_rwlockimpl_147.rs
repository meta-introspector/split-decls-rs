// Generated macro for impl_147 (impl)
macro_rules! Depcrate_rwlockimpl_147 {
() => {
// Module: crate::rwlock
// Provides: {"impl_147"}
// Dependencies: {}
impl < T : fmt :: Debug + ? Sized > fmt :: Debug for RwLockUpgradableReadGuardArc < T > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
