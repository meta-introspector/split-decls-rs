macro_rules! deps {
    () => {
        Env!();
    };
}

macro_rules! BackupEngine {
    () => {
        deps!();
        pub struct BackupEngine { inner : * mut ffi :: rocksdb_backup_engine_t , _outlive : Env , }
    };
}

BackupEngine!();