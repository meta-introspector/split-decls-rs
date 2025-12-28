macro_rules! deps {
    () => {
        ErrorKind!();
        ParseIntegerError!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl ParseIntegerError { fn desc (& self) -> & str { match self . kind { ErrorKind :: Empty => "cannot parse integer without digits" , ErrorKind :: InvalidDigit => "invalid digit found in slice" , ErrorKind :: Overflow => "number too large to fit in target type" , ErrorKind :: Underflow => "number too small to fit in target type" , } } }
    };
}

impl_20!();