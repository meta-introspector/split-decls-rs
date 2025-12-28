macro_rules! deps {
    () => {
        Zero!();
    };
}

macro_rules! not_zero_impl {
    () => {
        deps!();
        macro_rules ! not_zero_impl { ($ t : ty ,$ z : expr) => { impl Zero for $ t { fn zero () -> Self { $ z as $ t } # [allow (clippy :: float_cmp)] fn is_zero (& self) -> bool { self == & Self :: zero () } } } ; }
    };
}

not_zero_impl!();