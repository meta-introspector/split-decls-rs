macro_rules! deps {
    () => {
        CuckooTableOptions!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl Drop for CuckooTableOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_cuckoo_options_destroy (self . inner) ; } } }
    };
}

impl_188!();