macro_rules! deps {
    () => {
        SstFileWriter!();
    };
}

macro_rules! impl_405 {
    () => {
        deps!();
        impl Drop for SstFileWriter < '_ > { fn drop (& mut self) { unsafe { ffi :: rocksdb_sstfilewriter_destroy (self . inner) ; } } }
    };
}

impl_405!()