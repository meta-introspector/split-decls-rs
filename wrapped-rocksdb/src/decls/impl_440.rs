macro_rules! deps {
    () => {
        OptimisticTransactionOptions!();
    };
}

macro_rules! impl_440 {
    () => {
        deps!();
        unsafe impl Send for OptimisticTransactionOptions { }
    };
}

impl_440!();