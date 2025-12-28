macro_rules! deps {
    () => {
        TransactionOptions!();
    };
}

macro_rules! impl_432 {
    () => {
        deps!();
        impl Drop for TransactionOptions { fn drop (& mut self) { unsafe { ffi :: rocksdb_transaction_options_destroy (self . inner) ; } } }
    };
}

impl_432!()