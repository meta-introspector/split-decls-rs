macro_rules! deps {
    () => {
        Int!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < const LIMBS : usize > fmt :: Display for Int < LIMBS > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: UpperHex :: fmt (self , f) } }
    };
}

impl_119!()