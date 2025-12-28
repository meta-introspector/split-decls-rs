macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl core :: error :: Error for Error { }
    };
}

impl_7!();