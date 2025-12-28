macro_rules! deps {
    () => {
        CStringLit!();
        Buffer!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < B : Buffer > fmt :: Display for CStringLit < B > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad (& self . raw) } }
    };
}

impl_86!()