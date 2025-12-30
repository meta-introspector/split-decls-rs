// Generated macro for impl_245 (impl)
macro_rules! Depcrate_db_optionsimpl_245 {
() => {
// Module: crate::db_options
// Provides: {"impl_245"}
// Dependencies: {}
impl Drop for FlushOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_flushoptions_destroy (self . inner) ; } } }
};
}
