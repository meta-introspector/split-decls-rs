macro_rules! deps {
    () => {
        Limb!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl fmt :: Display for Limb { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: UpperHex :: fmt (self , f) } }
    };
}

impl_166!();