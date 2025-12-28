macro_rules! deps {
    () => {
        OptimisticTransactionOptions!();
    };
}

macro_rules! impl_444 {
    () => {
        deps!();
        impl Drop for OptimisticTransactionOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_optimistictransaction_options_destroy (self . inner) ; } } }
    };
}

impl_444!();