macro_rules! deps {
    () => {
        BackupEngine!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl Drop for BackupEngine { fn drop (& mut self) { unsafe { ffi :: rocksdb_backup_engine_close (self . inner) ; } } }
    };
}

impl_29!()