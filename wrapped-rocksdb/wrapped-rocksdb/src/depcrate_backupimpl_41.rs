// Generated macro for impl_41 (impl)
macro_rules! Depcrate_backupimpl_41 {
() => {
// Module: crate::backup
// Provides: {"impl_41"}
// Dependencies: {}
impl Drop for BackupEngine { fn drop (& mut self) { unsafe { ffi :: rocksdb_backup_engine_close (self . inner) ; } } }
};
}
