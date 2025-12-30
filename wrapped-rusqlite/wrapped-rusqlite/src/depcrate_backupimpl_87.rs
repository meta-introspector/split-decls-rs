// Generated macro for impl_87 (impl)
macro_rules! Depcrate_backupimpl_87 {
() => {
// Module: crate::backup
// Provides: {"impl_87"}
// Dependencies: {}
impl Drop for Backup < '_ , '_ > { # [inline] fn drop (& mut self) { unsafe { ffi :: sqlite3_backup_finish (self . b) } ; } }
};
}
