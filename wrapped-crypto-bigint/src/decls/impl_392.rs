macro_rules! deps {
    () => {
        Uint!();
    };
}

macro_rules! impl_392 {
    () => {
        deps!();
        impl < const LIMBS : usize > fmt :: UpperHex for Uint < LIMBS > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: UpperHex :: fmt (self . as_uint_ref () , f) } }
    };
}

impl_392!();