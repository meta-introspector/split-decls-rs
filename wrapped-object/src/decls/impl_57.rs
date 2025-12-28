macro_rules! deps {
    () => {
        Endian!();
        U16Bytes!();
        Result!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < E : Endian > fmt :: Debug for U16Bytes < E > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "U16({:x}, {:x})" , self . 0 [0] , self . 0 [1] ,) } }
    };
}

impl_57!()