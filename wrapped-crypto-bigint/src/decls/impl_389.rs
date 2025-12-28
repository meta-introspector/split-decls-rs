macro_rules! deps {
    () => {
        Uint!();
    };
}

macro_rules! impl_389 {
    () => {
        deps!();
        impl < const LIMBS : usize > fmt :: Binary for Uint < LIMBS > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Binary :: fmt (self . as_uint_ref () , f) } }
    };
}

impl_389!();