macro_rules! deps {
    () => {
        ParseError!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl Error for ParseError { }
    };
}

impl_25!()