macro_rules! deps {
    () => {
        FifoCompactOptions!();
    };
}

macro_rules! impl_225 {
    () => {
        deps!();
        impl Drop for FifoCompactOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_fifo_compaction_options_destroy (self . inner) ; } } }
    };
}

impl_225!()