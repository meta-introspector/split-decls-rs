macro_rules! deps {
    () => {
        NameParseError!();
    };
}

macro_rules! impl_408 {
    () => {
        deps!();
        impl core :: fmt :: Display for NameParseError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "unrecognized name") } }
    };
}

impl_408!();