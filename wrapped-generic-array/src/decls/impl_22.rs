macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < N : ArrayLength > fmt :: UpperHex for GenericArray < u8 , N > where N : Add < N > , Sum < N , N > : ArrayLength , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { generic_hex :: < _ , true > (self , f) } }
    };
}

impl_22!()