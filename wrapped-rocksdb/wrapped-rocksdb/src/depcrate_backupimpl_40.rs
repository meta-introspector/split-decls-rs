// Generated macro for impl_40 (impl)
macro_rules! Depcrate_backupimpl_40 {
() => {
// Module: crate::backup
// Provides: {"impl_40"}
// Dependencies: {}
impl Default for RestoreOptions { fn default () -> Self { unsafe { let opts = ffi :: rocksdb_restore_options_create () ; assert ! (! opts . is_null () , "Could not create RocksDB restore options") ; Self { inner : opts } } } }
};
}
