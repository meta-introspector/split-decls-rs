macro_rules! deps {
    () => {
        ParseError!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl std :: error :: Error for ParseError { }
    };
}

impl_30!()