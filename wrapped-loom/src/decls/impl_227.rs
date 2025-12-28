macro_rules! deps {
    () => {
        Lazy!();
    };
}

macro_rules! impl_227 {
    () => {
        deps!();
        impl < T : 'static > fmt :: Debug for Lazy < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("Lazy { .. }") } }
    };
}

impl_227!();