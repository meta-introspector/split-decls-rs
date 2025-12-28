macro_rules! deps {
    () => {
        Int!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < const LIMBS : usize > fmt :: UpperHex for Int < LIMBS > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: UpperHex :: fmt (& self . 0 , f) } }
    };
}

impl_121!();