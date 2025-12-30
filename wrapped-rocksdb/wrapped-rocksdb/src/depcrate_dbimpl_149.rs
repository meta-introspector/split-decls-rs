// Generated macro for impl_149 (impl)
macro_rules! Depcrate_dbimpl_149 {
() => {
// Module: crate::db
// Provides: {"impl_149"}
// Dependencies: {}
impl Drop for DBWithThreadModeInner { fn drop (& mut self) { unsafe { ffi :: rocksdb_close (self . inner) ; } } }
};
}
