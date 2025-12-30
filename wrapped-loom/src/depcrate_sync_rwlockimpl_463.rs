// Generated macro for impl_463 (impl)
macro_rules! Depcrate_sync_rwlockimpl_463 {
() => {
// Module: crate::sync::rwlock
// Provides: {"impl_463"}
// Dependencies: {}
impl < T : Default > Default for RwLock < T > { # [doc = " Creates a `RwLock<T>`, with the `Default` value for T."] fn default () -> Self { Self :: new (Default :: default ()) } }
};
}
