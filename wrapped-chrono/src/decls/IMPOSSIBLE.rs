macro_rules! deps {
    () => {
        ParseError!();
        ParseErrorKind!();
    };
}

macro_rules! IMPOSSIBLE {
    () => {
        deps!();
        const IMPOSSIBLE : ParseError = ParseError (ParseErrorKind :: Impossible) ;
    };
}

IMPOSSIBLE!();