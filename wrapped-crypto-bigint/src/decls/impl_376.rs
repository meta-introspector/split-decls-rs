macro_rules! deps {
    () => {
        Constants!();
        Uint!();
    };
}

macro_rules! impl_376 {
    () => {
        deps!();
        impl < const LIMBS : usize > Constants for Uint < LIMBS > { const MAX : Self = Self :: MAX ; }
    };
}

impl_376!()