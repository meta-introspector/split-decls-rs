macro_rules! deps {
    () => {
        ParseEditionError!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl fmt :: Display for ParseEditionError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "invalid edition: {:?}" , self . invalid_input) } }
    };
}

impl_4!();