macro_rules! deps {
    () => {
        Internable!();
        Interned!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < T : Display + Internable + ? Sized > Display for Interned < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* self . arc) . fmt (f) } }
    };
}

impl_42!();