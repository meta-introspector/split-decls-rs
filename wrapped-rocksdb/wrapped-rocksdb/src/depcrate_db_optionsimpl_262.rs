// Generated macro for impl_262 (impl)
macro_rules! Depcrate_db_optionsimpl_262 {
() => {
// Module: crate::db_options
// Provides: {"impl_262"}
// Dependencies: {}
impl Default for LruCacheOptions { fn default () -> Self { let inner = unsafe { ffi :: rocksdb_lru_cache_options_create () } ; assert ! (! inner . is_null () , "Could not create RocksDB LRU cache options") ; Self { inner } } }
};
}
