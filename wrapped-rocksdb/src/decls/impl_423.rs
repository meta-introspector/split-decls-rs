macro_rules! deps {
    () => {
        OptimisticTransactionDBInner!();
        DBInner!();
    };
}

macro_rules! impl_423 {
    () => {
        deps!();
        impl DBInner for OptimisticTransactionDBInner { fn inner (& self) -> * mut ffi :: rocksdb_t { self . base } }
    };
}

impl_423!();