macro_rules! deps {
    () => {
        BlockBasedOptions!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl Drop for BlockBasedOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_block_based_options_destroy (self . inner) ; } } }
    };
}

impl_187!()