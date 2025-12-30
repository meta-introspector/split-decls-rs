// Generated macro for impl_244 (impl)
macro_rules! Depcrate_db_optionsimpl_244 {
() => {
// Module: crate::db_options
// Provides: {"impl_244"}
// Dependencies: {}
impl Drop for CuckooTableOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_cuckoo_options_destroy (self . inner) ; } } }
};
}
