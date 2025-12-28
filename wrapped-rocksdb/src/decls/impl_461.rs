macro_rules! deps {
    () => {
        TransactionDB!();
        ThreadMode!();
    };
}

macro_rules! impl_461 {
    () => {
        deps!();
        impl < T : ThreadMode > Drop for TransactionDB < T > { fn drop (& mut self) { unsafe { self . prepared_transactions () . clear () ; self . cfs . drop_all_cfs_internal () ; ffi :: rocksdb_transactiondb_close (self . inner) ; } } }
    };
}

impl_461!();