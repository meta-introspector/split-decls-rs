macro_rules! deps {
    () => {
        FixedBitSet!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl BitAndAssign < & Self > for FixedBitSet { fn bitand_assign (& mut self , other : & Self) { self . intersect_with (other) ; } }
    };
}

impl_62!()