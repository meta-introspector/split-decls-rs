macro_rules! deps {
    () => {
        TransactionOptions!();
    };
}

macro_rules! impl_429 {
    () => {
        deps!();
        unsafe impl Sync for TransactionOptions { }
    };
}

impl_429!()