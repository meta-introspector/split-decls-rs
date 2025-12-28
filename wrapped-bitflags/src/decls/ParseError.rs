macro_rules! deps {
    () => {
        ParseErrorKind!();
    };
}

macro_rules! ParseError {
    () => {
        deps!();
        # [doc = " An error encountered while parsing flags from text."] # [derive (Debug)] pub struct ParseError (ParseErrorKind) ;
    };
}

ParseError!();