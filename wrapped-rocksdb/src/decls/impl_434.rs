macro_rules! deps {
    () => {
        TransactionDBOptions!();
    };
}

macro_rules! impl_434 {
    () => {
        deps!();
        unsafe impl Send for TransactionDBOptions { }
    };
}

impl_434!();