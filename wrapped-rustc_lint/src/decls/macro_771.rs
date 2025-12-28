macro_rules! deps {
    () => {
        ShadowedIntoIter!();
    };
}

macro_rules! macro_771 {
    () => {
        deps!();
        impl_lint_pass ! (ShadowedIntoIter => [ARRAY_INTO_ITER , BOXED_SLICE_INTO_ITER]) ;
    };
}

macro_771!();