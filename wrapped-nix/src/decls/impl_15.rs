macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl error :: Error for Errno { }
    };
}

impl_15!();