macro_rules! deps {
    () => {
        TransactionDBOptions!();
    };
}

macro_rules! impl_438 {
    () => {
        deps!();
        impl Drop for TransactionDBOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_transactiondb_options_destroy (self . inner) ; } } }
    };
}

impl_438!()