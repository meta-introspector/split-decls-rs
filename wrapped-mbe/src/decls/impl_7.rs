macro_rules! deps {
    () => {
        ParseError!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl ParseError { fn expected (e : & str) -> ParseError { ParseError :: Expected (e . into ()) } fn unexpected (e : & str) -> ParseError { ParseError :: UnexpectedToken (e . into ()) } }
    };
}

impl_7!()