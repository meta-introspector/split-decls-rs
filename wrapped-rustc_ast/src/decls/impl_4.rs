macro_rules! deps {
    () => {
        Lifetime!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl fmt :: Debug for Lifetime { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "lifetime({}: {})" , self . id , self) } }
    };
}

impl_4!()