// Generated macro for impl_157 (impl)
macro_rules! Depcrate_rwlockimpl_157 {
() => {
// Module: crate::rwlock
// Provides: {"impl_157"}
// Dependencies: {}
impl < T : fmt :: Debug + ? Sized > fmt :: Debug for RwLockWriteGuard < '_ , T > { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
