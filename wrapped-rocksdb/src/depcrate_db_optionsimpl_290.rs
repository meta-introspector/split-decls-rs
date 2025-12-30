// Generated macro for impl_290 (impl)
macro_rules! Depcrate_db_optionsimpl_290 {
() => {
// Module: crate::db_options
// Provides: {"impl_290"}
// Dependencies: {}
impl Default for CompactOptions { fn default () -> Self { let opts = unsafe { ffi :: rocksdb_compactoptions_create () } ; assert ! (! opts . is_null () , "Could not create RocksDB Compact Options") ; Self { inner : opts , full_history_ts_low : None , } } }
};
}
