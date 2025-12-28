macro_rules! deps {
    () => {
        FlushOptions!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl Drop for FlushOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_flushoptions_destroy (self . inner) ; } } }
    };
}

impl_189!()