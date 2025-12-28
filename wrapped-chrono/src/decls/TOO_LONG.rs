macro_rules! deps {
    () => {
        ParseErrorKind!();
        ParseError!();
    };
}

macro_rules! TOO_LONG {
    () => {
        deps!();
        pub (crate) const TOO_LONG : ParseError = ParseError (ParseErrorKind :: TooLong) ;
    };
}

TOO_LONG!()