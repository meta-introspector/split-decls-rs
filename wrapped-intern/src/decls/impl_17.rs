macro_rules! deps {
    () => {
        Interned!();
        Internable!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < T : Display + Internable + ? Sized > Display for Interned < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* self . arc) . fmt (f) } }
    };
}

impl_17!()