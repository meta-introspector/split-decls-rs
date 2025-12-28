macro_rules! deps {
    () => {
        Utf8Error!();
        Error!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl std :: error :: Error for Utf8Error { }
    };
}

impl_2!()