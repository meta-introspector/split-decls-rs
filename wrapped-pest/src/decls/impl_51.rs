macro_rules! deps {
    () => {
        Pairs!();
        RuleType!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < R : RuleType > ExactSizeIterator for Pairs < '_ , R > { # [inline] fn len (& self) -> usize { self . pairs_count } }
    };
}

impl_51!()