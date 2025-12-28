macro_rules! deps {
    () => {
        DotParsingError!();
    };
}

macro_rules! impl_580 {
    () => {
        deps!();
        impl Display for DotParsingError { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , core :: fmt :: Error > { write ! (f , "{}" , self . error) } }
    };
}

impl_580!();