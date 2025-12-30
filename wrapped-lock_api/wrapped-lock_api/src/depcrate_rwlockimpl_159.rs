// Generated macro for impl_159 (impl)
macro_rules! Depcrate_rwlockimpl_159 {
() => {
// Module: crate::rwlock
// Provides: {"impl_159"}
// Dependencies: {}
impl < 'a , R : RawRwLock + 'a , T : fmt :: Debug + ? Sized + 'a > fmt :: Debug for RwLockReadGuard < 'a , R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
