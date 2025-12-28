macro_rules! deps {
    () => {
        Backup!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl Drop for Backup < '_ , '_ > { # [inline] fn drop (& mut self) { unsafe { ffi :: sqlite3_backup_finish (self . b) } ; } }
    };
}

impl_32!()