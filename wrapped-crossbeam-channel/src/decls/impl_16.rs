macro_rules! deps {
    () => {
        Sender!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < T > fmt :: Debug for Sender < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("Sender { .. }") } }
    };
}

impl_16!();