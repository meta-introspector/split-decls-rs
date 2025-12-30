// Generated macro for impl_42 (impl)
macro_rules! Depcrate_backupimpl_42 {
() => {
// Module: crate::backup
// Provides: {"impl_42"}
// Dependencies: {}
impl Drop for BackupEngineOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_backup_engine_options_destroy (self . inner) ; } } }
};
}
