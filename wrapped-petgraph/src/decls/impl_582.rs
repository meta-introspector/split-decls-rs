macro_rules! deps {
    () => {
        DotParsingError!();
    };
}

macro_rules! impl_582 {
    () => {
        deps!();
        impl Error for DotParsingError { }
    };
}

impl_582!()