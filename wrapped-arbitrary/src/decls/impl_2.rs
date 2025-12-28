macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl error :: Error for Error { }
    };
}

impl_2!()