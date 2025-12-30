// Generated macro for impl_178 (impl)
macro_rules! Depcrate_rwlockimpl_178 {
() => {
// Module: crate::rwlock
// Provides: {"impl_178"}
// Dependencies: {}
impl < 'a , R : RawRwLock + 'a , T : fmt :: Debug + ? Sized + 'a > fmt :: Debug for RwLockWriteGuard < 'a , R , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (& * * self , f) } }
};
}
