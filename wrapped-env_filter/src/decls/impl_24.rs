macro_rules! deps {
    () => {
        ParseError!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl Display for ParseError { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "error parsing logger filter: {}" , self . details) } }
    };
}

impl_24!();