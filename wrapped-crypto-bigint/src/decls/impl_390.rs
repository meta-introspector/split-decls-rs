macro_rules! deps {
    () => {
        Uint!();
    };
}

macro_rules! impl_390 {
    () => {
        deps!();
        impl < const LIMBS : usize > fmt :: Display for Uint < LIMBS > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: UpperHex :: fmt (self , f) } }
    };
}

impl_390!()