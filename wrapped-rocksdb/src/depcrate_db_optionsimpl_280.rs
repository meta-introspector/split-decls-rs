// Generated macro for impl_280 (impl)
macro_rules! Depcrate_db_optionsimpl_280 {
() => {
// Module: crate::db_options
// Provides: {"impl_280"}
// Dependencies: {}
impl Default for FifoCompactOptions { fn default () -> Self { let opts = unsafe { ffi :: rocksdb_fifo_compaction_options_create () } ; assert ! (! opts . is_null () , "Could not create RocksDB Fifo Compaction Options") ; Self { inner : opts } } }
};
}
