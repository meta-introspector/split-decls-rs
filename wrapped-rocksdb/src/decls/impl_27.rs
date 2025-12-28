macro_rules! deps {
    () => {
        RestoreOptions!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl RestoreOptions { # [doc = " Sets `keep_log_files`. If true, restore won't overwrite the existing log files in wal_dir."] # [doc = " It will also move all log files from archive directory to wal_dir. Use this option in"] # [doc = " combination with BackupEngineOptions::backup_log_files = false for persisting in-memory"] # [doc = " databases."] # [doc = ""] # [doc = " Default: false"] pub fn set_keep_log_files (& mut self , keep_log_files : bool) { unsafe { ffi :: rocksdb_restore_options_set_keep_log_files (self . inner , i32 :: from (keep_log_files)) ; } } }
    };
}

impl_27!()