macro_rules! deps {
    () => {
        ParseError!();
        ParseErrorKind!();
    };
}

macro_rules! NOT_ENOUGH {
    () => {
        deps!();
        const NOT_ENOUGH : ParseError = ParseError (ParseErrorKind :: NotEnough) ;
    };
}

NOT_ENOUGH!();