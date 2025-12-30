// Generated macro for impl_299 (impl)
macro_rules! Depcrate_db_optionsimpl_299 {
() => {
// Module: crate::db_options
// Provides: {"impl_299"}
// Dependencies: {}
impl Drop for DBPath { fn drop (& mut self) { unsafe { ffi :: rocksdb_dbpath_destroy (self . inner) ; } } }
};
}
