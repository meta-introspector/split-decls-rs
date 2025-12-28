macro_rules! deps {
    () => {
        NonZero!();
    };
}

macro_rules! impl_212 {
    () => {
        deps!();
        impl < T > fmt :: UpperHex for NonZero < T > where T : fmt :: UpperHex + ? Sized , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: UpperHex :: fmt (& self . 0 , f) } }
    };
}

impl_212!()