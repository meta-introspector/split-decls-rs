// Generated macro for impl_281 (impl)
macro_rules! Depcrate_db_optionsimpl_281 {
() => {
// Module: crate::db_options
// Provides: {"impl_281"}
// Dependencies: {}
impl Drop for FifoCompactOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_fifo_compaction_options_destroy (self . inner) ; } } }
};
}
