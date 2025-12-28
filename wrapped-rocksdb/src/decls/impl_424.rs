macro_rules! deps {
    () => {
        OptimisticTransactionDBInner!();
    };
}

macro_rules! impl_424 {
    () => {
        deps!();
        impl Drop for OptimisticTransactionDBInner { fn drop (& mut self) { unsafe { ffi :: rocksdb_optimistictransactiondb_close_base_db (self . base) ; ffi :: rocksdb_optimistictransactiondb_close (self . db) ; } } }
    };
}

impl_424!();