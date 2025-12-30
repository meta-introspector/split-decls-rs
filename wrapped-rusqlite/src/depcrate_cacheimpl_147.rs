// Generated macro for impl_147 (impl)
macro_rules! Depcrate_cacheimpl_147 {
() => {
// Module: crate::cache
// Provides: {"impl_147"}
// Dependencies: {}
impl Drop for CachedStatement < '_ > { # [inline] fn drop (& mut self) { if let Some (stmt) = self . stmt . take () { self . cache . cache_stmt (unsafe { stmt . into_raw () }) ; } } }
};
}
