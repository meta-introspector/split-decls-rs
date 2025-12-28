macro_rules! deps {
    () => {
        DB!();
        Transaction!();
    };
}

macro_rules! impl_450 {
    () => {
        deps!();
        impl < DB > Drop for Transaction < '_ , DB > { fn drop (& mut self) { unsafe { ffi :: rocksdb_transaction_destroy (self . inner) ; } } }
    };
}

impl_450!();