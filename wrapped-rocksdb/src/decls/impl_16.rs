macro_rules! deps {
    () => {
        CSlice!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl Drop for CSlice { fn drop (& mut self) { unsafe { ffi :: rocksdb_free (self . data as * mut c_void) ; } } }
    };
}

impl_16!();