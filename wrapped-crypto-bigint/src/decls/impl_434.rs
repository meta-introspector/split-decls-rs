macro_rules! deps {
    () => {
        Wrapping!();
    };
}

macro_rules! impl_434 {
    () => {
        deps!();
        impl < T : fmt :: LowerHex > fmt :: LowerHex for Wrapping < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_434!()