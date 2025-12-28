macro_rules! deps {
    () => {
        FixedBitSet!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl BitOrAssign < & Self > for FixedBitSet { fn bitor_assign (& mut self , other : & Self) { self . union_with (other) ; } }
    };
}

impl_134!();