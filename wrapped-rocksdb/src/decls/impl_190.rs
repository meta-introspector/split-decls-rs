macro_rules! deps {
    () => {
        WriteOptions!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl Drop for WriteOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_writeoptions_destroy (self . inner) ; } } }
    };
}

impl_190!()