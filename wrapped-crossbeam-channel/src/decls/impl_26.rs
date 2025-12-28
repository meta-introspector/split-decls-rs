macro_rules! deps {
    () => {
        Receiver!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < T > fmt :: Debug for Receiver < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("Receiver { .. }") } }
    };
}

impl_26!()