macro_rules! deps {
    () => {
        ParseError!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl Error for ParseError < '_ > { }
    };
}

impl_6!()