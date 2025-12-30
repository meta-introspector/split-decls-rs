// Generated macro for impl_213 (impl)
macro_rules! Depcrate_utilimpl_213 {
() => {
// Module: crate::util
// Provides: {"impl_213"}
// Dependencies: {}
impl < R : RawRwLock > Drop for RwLockWriteGuardDetached < '_ , R > { fn drop (& mut self) { unsafe { self . lock . unlock_exclusive () ; } } }
};
}
