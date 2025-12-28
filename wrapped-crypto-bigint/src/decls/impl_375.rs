macro_rules! deps {
    () => {
        Bounded!();
        Uint!();
    };
}

macro_rules! impl_375 {
    () => {
        deps!();
        impl < const LIMBS : usize > Bounded for Uint < LIMBS > { const BITS : u32 = Self :: BITS ; const BYTES : usize = Self :: BYTES ; }
    };
}

impl_375!()