macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl error :: Error for Error { }
    };
}

impl_8!();