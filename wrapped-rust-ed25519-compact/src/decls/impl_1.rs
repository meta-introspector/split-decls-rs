macro_rules! deps {
    () => {
        Seed!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl From < [u8 ; 32] > for Seed { fn from (seed : [u8 ; 32]) -> Self { Seed (seed) } }
    };
}

impl_1!();