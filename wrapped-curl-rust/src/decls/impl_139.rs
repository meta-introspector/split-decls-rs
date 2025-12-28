macro_rules! deps {
    () => {
        Easy2Handle!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl < H : fmt :: Debug > fmt :: Debug for Easy2Handle < H > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . easy . fmt (f) } }
    };
}

impl_139!();