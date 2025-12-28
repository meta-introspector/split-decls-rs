macro_rules! deps {
    () => {
        TransactionOptions!();
    };
}

macro_rules! impl_428 {
    () => {
        deps!();
        unsafe impl Send for TransactionOptions { }
    };
}

impl_428!();