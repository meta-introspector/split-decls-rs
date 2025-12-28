macro_rules! deps {
    () => {
        Interned!();
        Internable!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < T : Debug + Internable + ? Sized > Debug for Interned < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { (* self . arc) . fmt (f) } }
    };
}

impl_16!()