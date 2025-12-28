macro_rules! deps {
    () => {
        Empty!();
    };
}

macro_rules! impl_395 {
    () => {
        deps!();
        impl < T : Send > fmt :: Debug for Empty < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("Empty") } }
    };
}

impl_395!()