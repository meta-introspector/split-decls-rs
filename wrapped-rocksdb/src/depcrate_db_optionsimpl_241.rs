// Generated macro for impl_241 (impl)
macro_rules! Depcrate_db_optionsimpl_241 {
() => {
// Module: crate::db_options
// Provides: {"impl_241"}
// Dependencies: {}
impl Drop for Options { fn drop (& mut self) { unsafe { ffi :: rocksdb_options_destroy (self . inner) ; } } }
};
}
