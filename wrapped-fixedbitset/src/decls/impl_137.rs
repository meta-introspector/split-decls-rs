macro_rules! deps {
    () => {
        FixedBitSet!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl BitXorAssign < & Self > for FixedBitSet { fn bitxor_assign (& mut self , other : & Self) { self . symmetric_difference_with (other) ; } }
    };
}

impl_137!();