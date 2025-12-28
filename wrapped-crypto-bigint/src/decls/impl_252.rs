macro_rules! deps {
    () => {
        Odd!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        impl < T > fmt :: UpperHex for Odd < T > where T : fmt :: UpperHex + ? Sized , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: UpperHex :: fmt (& self . 0 , f) } }
    };
}

impl_252!()