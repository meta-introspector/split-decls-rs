macro_rules! deps {
    () => {
        ParseError!();
        ParseErrorKind!();
    };
}

macro_rules! INVALID {
    () => {
        deps!();
        const INVALID : ParseError = ParseError (ParseErrorKind :: Invalid) ;
    };
}

INVALID!()