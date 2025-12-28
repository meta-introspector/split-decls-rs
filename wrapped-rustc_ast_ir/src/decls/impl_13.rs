macro_rules! deps {
    () => {
        UintTy!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl fmt :: Debug for UintTy { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . name_str ()) } }
    };
}

impl_13!();