// Generated macro for impl_256 (impl)
macro_rules! Depcrate_db_optionsimpl_256 {
() => {
// Module: crate::db_options
// Provides: {"impl_256"}
// Dependencies: {}
impl Default for Options { fn default () -> Self { unsafe { let opts = ffi :: rocksdb_options_create () ; assert ! (! opts . is_null () , "Could not create RocksDB options") ; Self { inner : opts , outlive : OptionsMustOutliveDB :: default () , } } } }
};
}
