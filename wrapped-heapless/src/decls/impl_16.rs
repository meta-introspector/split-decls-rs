macro_rules! deps {
    () => {
        ExtendError!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl Error for ExtendError { }
    };
}

impl_16!();