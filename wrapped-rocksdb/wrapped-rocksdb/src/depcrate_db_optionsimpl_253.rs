// Generated macro for impl_253 (impl)
macro_rules! Depcrate_db_optionsimpl_253 {
() => {
// Module: crate::db_options
// Provides: {"impl_253"}
// Dependencies: {}
impl Default for CuckooTableOptions { fn default () -> Self { let opts = unsafe { ffi :: rocksdb_cuckoo_options_create () } ; assert ! (! opts . is_null () , "Could not create RocksDB cuckoo options") ; Self { inner : opts } } }
};
}
