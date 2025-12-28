macro_rules! deps {
    () => {
        OptimisticTransactionOptions!();
    };
}

macro_rules! impl_441 {
    () => {
        deps!();
        unsafe impl Sync for OptimisticTransactionOptions { }
    };
}

impl_441!()