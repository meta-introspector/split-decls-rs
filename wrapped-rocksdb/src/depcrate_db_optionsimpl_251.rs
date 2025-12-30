// Generated macro for impl_251 (impl)
macro_rules! Depcrate_db_optionsimpl_251 {
() => {
// Module: crate::db_options
// Provides: {"impl_251"}
// Dependencies: {}
impl Default for BlockBasedOptions { fn default () -> Self { let block_opts = unsafe { ffi :: rocksdb_block_based_options_create () } ; assert ! (! block_opts . is_null () , "Could not create RocksDB block based options") ; Self { inner : block_opts , outlive : BlockBasedOptionsMustOutliveDB :: default () , } } }
};
}
