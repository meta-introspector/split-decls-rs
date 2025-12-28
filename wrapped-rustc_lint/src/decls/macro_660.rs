macro_rules! deps {
    () => {
        MultipleSupertraitUpcastable!();
    };
}

macro_rules! macro_660 {
    () => {
        deps!();
        declare_lint_pass ! (MultipleSupertraitUpcastable => [MULTIPLE_SUPERTRAIT_UPCASTABLE]) ;
    };
}

macro_660!()