// Generated macro for impl_43 (impl)
macro_rules! Depcrate_backupimpl_43 {
() => {
// Module: crate::backup
// Provides: {"impl_43"}
// Dependencies: {}
impl Drop for RestoreOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_restore_options_destroy (self . inner) ; } } }
};
}
