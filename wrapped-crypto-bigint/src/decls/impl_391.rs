macro_rules! deps {
    () => {
        Uint!();
    };
}

macro_rules! impl_391 {
    () => {
        deps!();
        impl < const LIMBS : usize > fmt :: LowerHex for Uint < LIMBS > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: LowerHex :: fmt (self . as_uint_ref () , f) } }
    };
}

impl_391!();