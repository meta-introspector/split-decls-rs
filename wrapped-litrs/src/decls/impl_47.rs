macro_rules! deps {
    () => {
        Buffer!();
        ByteStringLit!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < B : Buffer > fmt :: Display for ByteStringLit < B > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad (& self . raw) } }
    };
}

impl_47!();