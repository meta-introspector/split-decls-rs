macro_rules! deps {
    () => {
        NameParseError!();
        Error!();
    };
}

macro_rules! impl_409 {
    () => {
        deps!();
        impl std :: error :: Error for NameParseError { }
    };
}

impl_409!();