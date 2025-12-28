macro_rules! deps {
    () => {
        Stealer!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < T > fmt :: Debug for Stealer < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("Stealer { .. }") } }
    };
}

impl_21!()