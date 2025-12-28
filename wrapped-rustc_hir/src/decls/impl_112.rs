macro_rules! deps {
    () => {
        Lifetime!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl fmt :: Display for Lifetime { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . ident . name . fmt (f) } }
    };
}

impl_112!()