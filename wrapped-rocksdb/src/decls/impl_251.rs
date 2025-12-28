macro_rules! deps {
    () => {
        DBPinnableSlice!();
    };
}

macro_rules! impl_251 {
    () => {
        deps!();
        impl Drop for DBPinnableSlice < '_ > { fn drop (& mut self) { unsafe { ffi :: rocksdb_pinnableslice_destroy (self . ptr) ; } } }
    };
}

impl_251!();