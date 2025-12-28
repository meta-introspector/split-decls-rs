macro_rules! deps {
    () => {
        Int!();
        Bounded!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl < const LIMBS : usize > Bounded for Int < LIMBS > { const BITS : u32 = Self :: BITS ; const BYTES : usize = Self :: BYTES ; }
    };
}

impl_105!();