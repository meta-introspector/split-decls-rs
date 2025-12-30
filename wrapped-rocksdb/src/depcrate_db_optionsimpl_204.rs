// Generated macro for impl_204 (impl)
macro_rules! Depcrate_db_optionsimpl_204 {
() => {
// Module: crate::db_options
// Provides: {"impl_204"}
// Dependencies: {}
impl Drop for CacheWrapper { fn drop (& mut self) { unsafe { ffi :: rocksdb_cache_destroy (self . inner . as_ptr ()) ; } } }
};
}
