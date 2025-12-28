macro_rules! deps {
    () => {
        Wrapping!();
    };
}

macro_rules! impl_435 {
    () => {
        deps!();
        impl < T : fmt :: UpperHex > fmt :: UpperHex for Wrapping < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_435!()