macro_rules! deps {
    () => {
        Collector!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl fmt :: Debug for Collector { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("Collector { .. }") } }
    };
}

impl_72!()