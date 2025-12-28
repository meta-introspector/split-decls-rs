macro_rules! deps {
    () => {
        Int!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < const LIMBS : usize > fmt :: Binary for Int < LIMBS > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Binary :: fmt (& self . 0 , f) } }
    };
}

impl_118!();