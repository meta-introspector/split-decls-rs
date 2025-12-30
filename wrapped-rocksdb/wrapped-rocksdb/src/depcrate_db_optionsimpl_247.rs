// Generated macro for impl_247 (impl)
macro_rules! Depcrate_db_optionsimpl_247 {
() => {
// Module: crate::db_options
// Provides: {"impl_247"}
// Dependencies: {}
impl Drop for LruCacheOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_lru_cache_options_destroy (self . inner) ; } } }
};
}
