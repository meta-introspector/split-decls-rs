// Generated macro for impl_150 (impl)
macro_rules! Depcrate_rwlockimpl_150 {
() => {
// Module: crate::rwlock
// Provides: {"impl_150"}
// Dependencies: {}
impl < R : RawRwLock , T : ? Sized + Default > Default for RwLock < R , T > { # [inline] fn default () -> RwLock < R , T > { RwLock :: new (Default :: default ()) } }
};
}
