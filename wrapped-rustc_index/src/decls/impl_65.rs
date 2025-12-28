macro_rules! deps {
    () => {
        FiniteBitSet!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl std :: fmt :: Debug for FiniteBitSet < u32 > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{:032b}" , self . 0) } }
    };
}

impl_65!();