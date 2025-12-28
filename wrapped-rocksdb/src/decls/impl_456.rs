macro_rules! deps {
    () => {
        ThreadMode!();
        TransactionDB!();
    };
}

macro_rules! impl_456 {
    () => {
        deps!();
        unsafe impl < T : ThreadMode > Sync for TransactionDB < T > { }
    };
}

impl_456!()