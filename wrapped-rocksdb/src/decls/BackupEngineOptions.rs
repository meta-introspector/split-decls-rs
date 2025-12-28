macro_rules! BackupEngineOptions {
    () => {
        pub struct BackupEngineOptions { inner : * mut ffi :: rocksdb_backup_engine_options_t , }
    };
}

BackupEngineOptions!()