macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < T > fmt :: Debug for IntoIter < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("IntoIter { .. }") } }
    };
}

impl_39!()