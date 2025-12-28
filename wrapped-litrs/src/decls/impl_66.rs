macro_rules! deps {
    () => {
        Buffer!();
        CharLit!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < B : Buffer > fmt :: Display for CharLit < B > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad (& self . raw) } }
    };
}

impl_66!()