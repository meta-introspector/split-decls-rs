macro_rules! deps {
    () => {
        ParseErrorKind!();
        ParseError!();
    };
}

macro_rules! INVALID {
    () => {
        deps!();
        const INVALID : ParseError = ParseError (ParseErrorKind :: Invalid) ;
    };
}

INVALID!();