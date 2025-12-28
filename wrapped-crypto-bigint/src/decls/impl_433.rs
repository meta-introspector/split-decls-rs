macro_rules! deps {
    () => {
        Wrapping!();
    };
}

macro_rules! impl_433 {
    () => {
        deps!();
        impl < T : fmt :: Octal > fmt :: Octal for Wrapping < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_433!()