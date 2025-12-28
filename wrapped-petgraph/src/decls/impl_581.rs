macro_rules! deps {
    () => {
        DotParsingError!();
    };
}

macro_rules! impl_581 {
    () => {
        deps!();
        impl From < ParsingError > for DotParsingError { fn from (error : ParsingError) -> Self { Self { error : Box :: new (error) , } } }
    };
}

impl_581!()