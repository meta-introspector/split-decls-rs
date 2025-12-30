// Generated macro for RwLockReadGuardArc (struct)
macro_rules! Depcrate_rwlockRwLockReadGuardArc {
() => {
// Module: crate::rwlock
// Provides: {"RwLockReadGuardArc"}
// Dependencies: {}
# [doc = " An owned, reference-counting guard that releases the read lock when dropped."] # [clippy :: has_significant_drop] pub struct RwLockReadGuardArc < T > { # [doc = " **WARNING**: This doesn't actually point to a `T`!"] # [doc = " It points to a `RwLock<T>`, via a pointer obtained with `Arc::into_raw`."] # [doc = " We lie for covariance."] lock : NonNull < T > , }
};
}
