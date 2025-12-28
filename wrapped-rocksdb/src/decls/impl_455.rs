macro_rules! deps {
    () => {
        ThreadMode!();
        TransactionDB!();
    };
}

macro_rules! impl_455 {
    () => {
        deps!();
        unsafe impl < T : ThreadMode > Send for TransactionDB < T > { }
    };
}

impl_455!()