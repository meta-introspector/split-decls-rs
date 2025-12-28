macro_rules! deps {
    () => {
        TransactionDBOptions!();
    };
}

macro_rules! impl_435 {
    () => {
        deps!();
        unsafe impl Sync for TransactionDBOptions { }
    };
}

impl_435!();