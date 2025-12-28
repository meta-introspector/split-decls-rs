macro_rules! deps {
    () => {
        ParseError!();
        ParseErrorKind!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl ParseError { pub fn new (orig : & str , kind : ParseErrorKind) -> ParseError { ParseError { kind , orig : orig . to_string () , } } }
    };
}

impl_31!()