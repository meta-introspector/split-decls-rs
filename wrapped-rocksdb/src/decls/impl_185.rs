macro_rules! deps {
    () => {
        Options!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        impl Drop for Options { fn drop (& mut self) { unsafe { ffi :: rocksdb_options_destroy (self . inner) ; } } }
    };
}

impl_185!();