macro_rules! deps {
    () => {
        ParseErrorKind!();
        ParseError!();
    };
}

macro_rules! IMPOSSIBLE {
    () => {
        deps!();
        const IMPOSSIBLE : ParseError = ParseError (ParseErrorKind :: Impossible) ;
    };
}

IMPOSSIBLE!()