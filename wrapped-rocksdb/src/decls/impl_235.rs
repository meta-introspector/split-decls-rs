macro_rules! deps {
    () => {
        CompactOptions!();
    };
}

macro_rules! impl_235 {
    () => {
        deps!();
        impl Drop for CompactOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_compactoptions_destroy (self . inner) ; } } }
    };
}

impl_235!()