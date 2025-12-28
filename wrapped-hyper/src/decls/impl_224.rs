macro_rules! deps {
    () => {
        UpgradeExpected!();
    };
}

macro_rules! impl_224 {
    () => {
        deps!();
        impl StdError for UpgradeExpected { }
    };
}

impl_224!();