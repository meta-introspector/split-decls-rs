macro_rules! deps {
    () => {
        RetryFailError!();
    };
}

macro_rules! impl_317 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for RetryFailError { }
    };
}

impl_317!();