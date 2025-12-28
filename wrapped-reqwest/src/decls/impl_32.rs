macro_rules! deps {
    () => {
        BadScheme!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl StdError for BadScheme { }
    };
}

impl_32!();