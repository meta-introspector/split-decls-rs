macro_rules! deps {
    () => {
        BackupEngineOptions!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl Drop for BackupEngineOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_backup_engine_options_destroy (self . inner) ; } } }
    };
}

impl_30!();