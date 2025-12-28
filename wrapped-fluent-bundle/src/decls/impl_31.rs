macro_rules! deps {
    () => {
        FluentError!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl From < ParserError > for FluentError { fn from (error : ParserError) -> Self { Self :: ParserError (error) } }
    };
}

impl_31!()