macro_rules! deps {
    () => {
        Endian!();
        Result!();
        U32Bytes!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl < E : Endian > fmt :: Debug for U32Bytes < E > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "U32({:x}, {:x}, {:x}, {:x})" , self . 0 [0] , self . 0 [1] , self . 0 [2] , self . 0 [3] ,) } }
    };
}

impl_58!()