macro_rules! deps {
    () => {
        DBPath!();
    };
}

macro_rules! impl_243 {
    () => {
        deps!();
        impl Drop for DBPath { fn drop (& mut self) { unsafe { ffi :: rocksdb_dbpath_destroy (self . inner) ; } } }
    };
}

impl_243!();