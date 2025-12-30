// Generated macro for impl_260 (impl)
macro_rules! Depcrate_db_optionsimpl_260 {
() => {
// Module: crate::db_options
// Provides: {"impl_260"}
// Dependencies: {}
impl Default for WriteOptions { fn default () -> Self { let write_opts = unsafe { ffi :: rocksdb_writeoptions_create () } ; assert ! (! write_opts . is_null () , "Could not create RocksDB write options") ; Self { inner : write_opts } } }
};
}
