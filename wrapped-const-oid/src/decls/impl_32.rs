macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl core :: error :: Error for Error { }
    };
}

impl_32!();