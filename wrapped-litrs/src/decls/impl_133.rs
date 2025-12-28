macro_rules! deps {
    () => {
        Buffer!();
        FloatLit!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl < B : Buffer > fmt :: Display for FloatLit < B > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , &* self . raw) } }
    };
}

impl_133!();