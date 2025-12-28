macro_rules! deps {
    () => {
        IntTy!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl fmt :: Debug for IntTy { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . name_str ()) } }
    };
}

impl_10!();