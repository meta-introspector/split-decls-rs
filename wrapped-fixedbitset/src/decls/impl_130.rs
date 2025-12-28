macro_rules! deps {
    () => {
        FixedBitSet!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl BitAndAssign for FixedBitSet { fn bitand_assign (& mut self , other : Self) { self . intersect_with (& other) ; } }
    };
}

impl_130!();