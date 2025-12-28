macro_rules! deps {
    () => {
        ParseErrorKind!();
    };
}

macro_rules! ParseError {
    () => {
        deps!();
        # [derive (Debug)] pub struct ParseError { kind : ParseErrorKind , orig : String , }
    };
}

ParseError!()