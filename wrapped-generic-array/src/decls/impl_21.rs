macro_rules! deps {
    () => {
        GenericArray!();
        ArrayLength!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < N : ArrayLength > fmt :: LowerHex for GenericArray < u8 , N > where N : Add < N > , Sum < N , N > : ArrayLength , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { generic_hex :: < _ , false > (self , f) } }
    };
}

impl_21!()