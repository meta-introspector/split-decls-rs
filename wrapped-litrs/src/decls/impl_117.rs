macro_rules! deps {
    () => {
        ParseError!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl std :: error :: Error for ParseError { }
    };
}

impl_117!();