macro_rules! deps {
    () => {
        RestoreOptions!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl Drop for RestoreOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_restore_options_destroy (self . inner) ; } } }
    };
}

impl_31!()