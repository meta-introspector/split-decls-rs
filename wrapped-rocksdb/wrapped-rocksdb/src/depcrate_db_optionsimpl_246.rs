// Generated macro for impl_246 (impl)
macro_rules! Depcrate_db_optionsimpl_246 {
() => {
// Module: crate::db_options
// Provides: {"impl_246"}
// Dependencies: {}
impl Drop for WriteOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_writeoptions_destroy (self . inner) ; } } }
};
}
