macro_rules! deps {
    () => {
        RuleType!();
        FlatPairs!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < R : RuleType > ExactSizeIterator for FlatPairs < '_ , R > { fn len (& self) -> usize { (self . end - self . start) >> 1 } }
    };
}

impl_25!()