macro_rules! deps {
    () => {
        EnvWrapper!();
    };
}

macro_rules! impl_256 {
    () => {
        deps!();
        impl Drop for EnvWrapper { fn drop (& mut self) { unsafe { ffi :: rocksdb_env_destroy (self . inner) ; } } }
    };
}

impl_256!()