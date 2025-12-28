macro_rules! deps {
    () => {
        ReadOptions!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl Drop for ReadOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_readoptions_destroy (self . inner) ; } } }
    };
}

impl_192!();