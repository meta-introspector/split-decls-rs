macro_rules! deps {
    () => {
        CharLit!();
        Buffer!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < B : Buffer > fmt :: Display for CharLit < B > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad (& self . raw) } }
    };
}

impl_66!();