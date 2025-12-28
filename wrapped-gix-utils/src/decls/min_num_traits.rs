macro_rules! deps {
    () => {
        MinNumTraits!();
    };
}

macro_rules! min_num_traits {
    () => {
        deps!();
        macro_rules ! min_num_traits { ($ t : ty) => { impl MinNumTraits for $ t { const ZERO : Self = 0 ; impl_checked ! (checked_add) ; impl_checked ! (checked_mul) ; impl_checked ! (checked_sub) ; } } ; }
    };
}

min_num_traits!();