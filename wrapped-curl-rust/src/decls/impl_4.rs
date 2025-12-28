macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl error :: Error for Error { }
    };
}

impl_4!()