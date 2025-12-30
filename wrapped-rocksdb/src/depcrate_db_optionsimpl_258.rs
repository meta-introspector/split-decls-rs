// Generated macro for impl_258 (impl)
macro_rules! Depcrate_db_optionsimpl_258 {
() => {
// Module: crate::db_options
// Provides: {"impl_258"}
// Dependencies: {}
impl Default for FlushOptions { fn default () -> Self { let flush_opts = unsafe { ffi :: rocksdb_flushoptions_create () } ; assert ! (! flush_opts . is_null () , "Could not create RocksDB flush options") ; Self { inner : flush_opts } } }
};
}
