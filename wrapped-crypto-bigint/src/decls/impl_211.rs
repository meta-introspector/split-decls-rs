macro_rules! deps {
    () => {
        NonZero!();
    };
}

macro_rules! impl_211 {
    () => {
        deps!();
        impl < T > fmt :: LowerHex for NonZero < T > where T : fmt :: LowerHex + ? Sized , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: LowerHex :: fmt (& self . 0 , f) } }
    };
}

impl_211!()