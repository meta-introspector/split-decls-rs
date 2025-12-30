// Generated macro for impl_151 (impl)
macro_rules! Depcrate_rwlockimpl_151 {
() => {
// Module: crate::rwlock
// Provides: {"impl_151"}
// Dependencies: {}
impl < R : RawRwLock , T > From < T > for RwLock < R , T > { # [inline] fn from (t : T) -> RwLock < R , T > { RwLock :: new (t) } }
};
}
