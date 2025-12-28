macro_rules! deps {
    () => {
        ParseError!();
        ParseErrorKind!();
        OutOfRange!();
    };
}

macro_rules! OUT_OF_RANGE {
    () => {
        deps!();
        pub (crate) const OUT_OF_RANGE : ParseError = ParseError (ParseErrorKind :: OutOfRange) ;
    };
}

OUT_OF_RANGE!()