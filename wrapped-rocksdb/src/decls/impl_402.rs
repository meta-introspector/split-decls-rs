macro_rules! deps {
    () => {
        EnvOptions!();
    };
}

macro_rules! impl_402 {
    () => {
        deps!();
        impl Drop for EnvOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_envoptions_destroy (self . inner) ; } } }
    };
}

impl_402!();