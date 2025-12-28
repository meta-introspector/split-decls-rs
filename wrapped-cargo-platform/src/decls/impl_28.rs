macro_rules! deps {
    () => {
        ParseError!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl fmt :: Display for ParseError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "failed to parse `{}` as a cfg expression: {}" , self . orig , self . kind) } }
    };
}

impl_28!();