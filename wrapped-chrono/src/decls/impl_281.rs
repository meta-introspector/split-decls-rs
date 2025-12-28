macro_rules! deps {
    () => {
        ParseError!();
        OutOfRange!();
        ParseErrorKind!();
    };
}

macro_rules! impl_281 {
    () => {
        deps!();
        impl fmt :: Display for ParseError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self . 0 { ParseErrorKind :: OutOfRange => write ! (f , "input is out of range") , ParseErrorKind :: Impossible => write ! (f , "no possible date and time matching input") , ParseErrorKind :: NotEnough => write ! (f , "input is not enough for unique date and time") , ParseErrorKind :: Invalid => write ! (f , "input contains invalid characters") , ParseErrorKind :: TooShort => write ! (f , "premature end of input") , ParseErrorKind :: TooLong => write ! (f , "trailing input") , ParseErrorKind :: BadFormat => write ! (f , "bad or unsupported format string") , _ => unreachable ! () , } } }
    };
}

impl_281!();