macro_rules! deps {
    () => {
        MacError!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl core :: error :: Error for MacError { }
    };
}

impl_70!();