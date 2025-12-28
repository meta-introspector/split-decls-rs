macro_rules! deps {
    () => {
        StringLit!();
        Buffer!();
    };
}

macro_rules! impl_226 {
    () => {
        deps!();
        impl < B : Buffer > fmt :: Display for StringLit < B > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad (& self . raw) } }
    };
}

impl_226!()