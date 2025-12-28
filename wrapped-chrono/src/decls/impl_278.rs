macro_rules! deps {
    () => {
        ParseErrorKind!();
        ParseError!();
    };
}

macro_rules! impl_278 {
    () => {
        deps!();
        impl ParseError { # [doc = " The category of parse error"] pub const fn kind (& self) -> ParseErrorKind { self . 0 } }
    };
}

impl_278!()