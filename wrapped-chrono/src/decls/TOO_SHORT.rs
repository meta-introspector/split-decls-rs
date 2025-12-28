macro_rules! deps {
    () => {
        ParseErrorKind!();
        ParseError!();
    };
}

macro_rules! TOO_SHORT {
    () => {
        deps!();
        const TOO_SHORT : ParseError = ParseError (ParseErrorKind :: TooShort) ;
    };
}

TOO_SHORT!()