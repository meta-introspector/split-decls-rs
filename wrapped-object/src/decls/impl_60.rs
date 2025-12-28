macro_rules! deps {
    () => {
        Result!();
        I16Bytes!();
        Endian!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < E : Endian > fmt :: Debug for I16Bytes < E > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "I16({:x}, {:x})" , self . 0 [0] , self . 0 [1] ,) } }
    };
}

impl_60!()