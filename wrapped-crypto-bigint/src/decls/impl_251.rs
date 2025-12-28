macro_rules! deps {
    () => {
        Odd!();
    };
}

macro_rules! impl_251 {
    () => {
        deps!();
        impl < T > fmt :: LowerHex for Odd < T > where T : fmt :: LowerHex + ? Sized , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: LowerHex :: fmt (& self . 0 , f) } }
    };
}

impl_251!()