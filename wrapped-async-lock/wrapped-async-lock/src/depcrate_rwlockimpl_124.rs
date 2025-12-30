// Generated macro for impl_124 (impl)
macro_rules! Depcrate_rwlockimpl_124 {
() => {
// Module: crate::rwlock
// Provides: {"impl_124"}
// Dependencies: {}
impl < T : fmt :: Debug + ? Sized > fmt :: Debug for RwLockReadGuard < '_ , T > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
