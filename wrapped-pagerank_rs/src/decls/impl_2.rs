macro_rules! deps {
    () => {
        PagerankError!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl Error for PagerankError { }
    };
}

impl_2!();