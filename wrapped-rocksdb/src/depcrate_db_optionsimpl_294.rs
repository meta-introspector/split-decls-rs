// Generated macro for impl_294 (impl)
macro_rules! Depcrate_db_optionsimpl_294 {
() => {
// Module: crate::db_options
// Provides: {"impl_294"}
// Dependencies: {}
impl Default for WaitForCompactOptions { fn default () -> Self { let opts = unsafe { ffi :: rocksdb_wait_for_compact_options_create () } ; assert ! (! opts . is_null () , "Could not create RocksDB Wait For Compact Options") ; Self { inner : opts } } }
};
}
