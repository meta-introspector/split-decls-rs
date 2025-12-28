macro_rules! deps {
    () => {
        TryIter!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < T > fmt :: Debug for TryIter < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("TryIter { .. }") } }
    };
}

impl_35!()