macro_rules! deps {
    () => {
        Endian!();
        I32Bytes!();
        Result!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < E : Endian > fmt :: Debug for I32Bytes < E > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "I32({:x}, {:x}, {:x}, {:x})" , self . 0 [0] , self . 0 [1] , self . 0 [2] , self . 0 [3] ,) } }
    };
}

impl_61!();