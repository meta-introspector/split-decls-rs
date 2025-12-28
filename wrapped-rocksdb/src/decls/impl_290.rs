macro_rules! deps {
    () => {
        PerfContext!();
    };
}

macro_rules! impl_290 {
    () => {
        deps!();
        impl Drop for PerfContext { fn drop (& mut self) { unsafe { ffi :: rocksdb_perfcontext_destroy (self . inner) ; } } }
    };
}

impl_290!();