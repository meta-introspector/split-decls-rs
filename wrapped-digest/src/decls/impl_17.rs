macro_rules! deps {
    () => {
        MacError!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl core :: error :: Error for MacError { }
    };
}

impl_17!()