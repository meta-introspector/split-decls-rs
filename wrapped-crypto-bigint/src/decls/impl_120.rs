macro_rules! deps {
    () => {
        Int!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < const LIMBS : usize > fmt :: LowerHex for Int < LIMBS > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: LowerHex :: fmt (& self . 0 , f) } }
    };
}

impl_120!()