// Generated macro for impl_215 (impl)
macro_rules! Depcrate_rwlockimpl_215 {
() => {
// Module: crate::rwlock
// Provides: {"impl_215"}
// Dependencies: {}
unsafe impl < 'a , R : RawRwLock + 'a , T : ? Sized + Sync + 'a > Send for MappedRwLockReadGuard < 'a , R , T > where R :: GuardMarker : Send { }
};
}
