macro_rules! deps {
    () => {
        Uint!();
    };
}

macro_rules! impl_377 {
    () => {
        deps!();
        impl < const LIMBS : usize > Default for Uint < LIMBS > { fn default () -> Self { Self :: ZERO } }
    };
}

impl_377!()