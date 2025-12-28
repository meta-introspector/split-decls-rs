macro_rules! deps {
    () => {
        CtOutput!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl < T : OutputSizeUser > fmt :: Debug for CtOutput < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("CtOutput { ... }") } }
    };
}

impl_65!();