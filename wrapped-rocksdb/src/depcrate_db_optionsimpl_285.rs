// Generated macro for impl_285 (impl)
macro_rules! Depcrate_db_optionsimpl_285 {
() => {
// Module: crate::db_options
// Provides: {"impl_285"}
// Dependencies: {}
impl Default for UniversalCompactOptions { fn default () -> Self { let opts = unsafe { ffi :: rocksdb_universal_compaction_options_create () } ; assert ! (! opts . is_null () , "Could not create RocksDB Universal Compaction Options") ; Self { inner : opts } } }
};
}
