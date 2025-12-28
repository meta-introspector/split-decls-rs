macro_rules! deps {
    () => {
        FloatTy!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl fmt :: Debug for FloatTy { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . name_str ()) } }
    };
}

impl_16!();