macro_rules! deps {
    () => {
        UniversalCompactOptions!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl Drop for UniversalCompactOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_universal_compaction_options_destroy (self . inner) ; } } }
    };
}

impl_230!();