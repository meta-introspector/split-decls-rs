// Generated macro for Backup (struct)
macro_rules! Depcrate_backupBackup {
() => {
// Module: crate::backup
// Provides: {"Backup"}
// Dependencies: {}
# [doc = " A handle to an online backup."] pub struct Backup < 'a , 'b > { phantom_from : PhantomData < & 'a Connection > , to : & 'b Connection , b : * mut ffi :: sqlite3_backup , }
};
}
