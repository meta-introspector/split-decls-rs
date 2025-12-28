macro_rules! deps {
    () => {
        BitMaskIter!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Iterator for BitMaskIter { type Item = usize ; # [inline] fn next (& mut self) -> Option < usize > { let bit = self . 0 . lowest_set_bit () ? ; self . 0 = self . 0 . remove_lowest_bit () ; Some (bit) } }
    };
}

impl_11!()