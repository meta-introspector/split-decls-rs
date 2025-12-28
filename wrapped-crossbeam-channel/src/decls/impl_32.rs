macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < T > fmt :: Debug for Iter < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("Iter { .. }") } }
    };
}

impl_32!();