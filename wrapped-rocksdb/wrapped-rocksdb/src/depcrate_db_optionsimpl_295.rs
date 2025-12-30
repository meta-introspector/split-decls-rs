// Generated macro for impl_295 (impl)
macro_rules! Depcrate_db_optionsimpl_295 {
() => {
// Module: crate::db_options
// Provides: {"impl_295"}
// Dependencies: {}
impl Drop for WaitForCompactOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_wait_for_compact_options_destroy (self . inner) ; } } }
};
}
