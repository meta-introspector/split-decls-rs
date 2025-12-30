// Generated macro for impl_225 (impl)
macro_rules! Depcrate_rwlockimpl_225 {
() => {
// Module: crate::rwlock
// Provides: {"impl_225"}
// Dependencies: {}
unsafe impl < 'a , R : RawRwLock + 'a , T : ? Sized + Send + 'a > Send for MappedRwLockWriteGuard < 'a , R , T > where R :: GuardMarker : Send { }
};
}
