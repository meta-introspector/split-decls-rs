macro_rules! deps {
    () => {
        Lifetime!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl fmt :: Display for Lifetime { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . ident . name) } }
    };
}

impl_5!();