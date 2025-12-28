macro_rules! deps {
    () => {
        Bounded!();
        Limb!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl Bounded for Limb { const BITS : u32 = Self :: BITS ; const BYTES : usize = Self :: BYTES ; }
    };
}

impl_156!()