macro_rules! deps {
    () => {
        ByteLit!();
        Buffer!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < B : Buffer > fmt :: Display for ByteLit < B > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad (& self . raw) } }
    };
}

impl_30!()