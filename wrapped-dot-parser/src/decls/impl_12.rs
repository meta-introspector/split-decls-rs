macro_rules! deps {
    () => {
        ParseError!();
        GraphFromFileError!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < 'a > From < ParseError < 'a > > for GraphFromFileError < 'a > { fn from (e : ParseError < 'a >) -> Self { Self :: ParseError (e) } }
    };
}

impl_12!()