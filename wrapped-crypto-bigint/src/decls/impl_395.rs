macro_rules! deps {
    () => {
        Uint!();
    };
}

macro_rules! impl_395 {
    () => {
        deps!();
        # [cfg (feature = "zeroize")] impl < const LIMBS : usize > DefaultIsZeroes for Uint < LIMBS > { }
    };
}

impl_395!()