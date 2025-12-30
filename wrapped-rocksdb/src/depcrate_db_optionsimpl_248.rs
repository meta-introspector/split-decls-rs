// Generated macro for impl_248 (impl)
macro_rules! Depcrate_db_optionsimpl_248 {
() => {
// Module: crate::db_options
// Provides: {"impl_248"}
// Dependencies: {}
impl Drop for ReadOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_readoptions_destroy (self . inner) ; } } }
};
}
