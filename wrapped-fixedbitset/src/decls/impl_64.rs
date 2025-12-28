macro_rules! deps {
    () => {
        FixedBitSet!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl BitOrAssign for FixedBitSet { fn bitor_assign (& mut self , other : Self) { self . union_with (& other) ; } }
    };
}

impl_64!()