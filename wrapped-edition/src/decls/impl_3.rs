macro_rules! deps {
    () => {
        ParseEditionError!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl std :: error :: Error for ParseEditionError { }
    };
}

impl_3!();