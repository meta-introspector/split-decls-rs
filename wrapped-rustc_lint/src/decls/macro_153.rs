macro_rules! deps {
    () => {
        DefaultCouldBeDerived!();
    };
}

macro_rules! macro_153 {
    () => {
        deps!();
        impl_lint_pass ! (DefaultCouldBeDerived => [DEFAULT_OVERRIDES_DEFAULT_FIELDS]) ;
    };
}

macro_153!()