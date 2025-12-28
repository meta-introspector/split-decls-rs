macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl std :: error :: Error for Error { }
    };
}

impl_6!()