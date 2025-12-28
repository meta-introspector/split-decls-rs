macro_rules! deps {
    () => {
        ParseError!();
        ParseErrorKind!();
    };
}

macro_rules! BAD_FORMAT {
    () => {
        deps!();
        const BAD_FORMAT : ParseError = ParseError (ParseErrorKind :: BadFormat) ;
    };
}

BAD_FORMAT!()