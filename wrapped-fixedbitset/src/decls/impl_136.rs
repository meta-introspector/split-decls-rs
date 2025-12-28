macro_rules! deps {
    () => {
        FixedBitSet!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl BitXorAssign for FixedBitSet { fn bitxor_assign (& mut self , other : Self) { self . symmetric_difference_with (& other) ; } }
    };
}

impl_136!();