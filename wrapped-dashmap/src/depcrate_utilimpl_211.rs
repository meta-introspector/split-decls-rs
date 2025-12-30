// Generated macro for impl_211 (impl)
macro_rules! Depcrate_utilimpl_211 {
() => {
// Module: crate::util
// Provides: {"impl_211"}
// Dependencies: {}
impl < R : RawRwLock > Drop for RwLockReadGuardDetached < '_ , R > { fn drop (& mut self) { unsafe { self . lock . unlock_shared () ; } } }
};
}
