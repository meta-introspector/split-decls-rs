macro_rules! deps {
    () => {
        Buffer!();
        IntegerLit!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl < B : Buffer > fmt :: Display for IntegerLit < B > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , &* self . raw) } }
    };
}

impl_189!()