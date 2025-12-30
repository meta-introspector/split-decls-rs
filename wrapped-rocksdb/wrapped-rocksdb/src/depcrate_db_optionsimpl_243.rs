// Generated macro for impl_243 (impl)
macro_rules! Depcrate_db_optionsimpl_243 {
() => {
// Module: crate::db_options
// Provides: {"impl_243"}
// Dependencies: {}
impl Drop for BlockBasedOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_block_based_options_destroy (self . inner) ; } } }
};
}
