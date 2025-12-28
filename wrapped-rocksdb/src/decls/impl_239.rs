macro_rules! deps {
    () => {
        WaitForCompactOptions!();
    };
}

macro_rules! impl_239 {
    () => {
        deps!();
        impl Drop for WaitForCompactOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_wait_for_compact_options_destroy (self . inner) ; } } }
    };
}

impl_239!()