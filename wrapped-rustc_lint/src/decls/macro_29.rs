macro_rules! deps {
    () => {
        MissingDoc!();
    };
}

macro_rules! macro_29 {
    () => {
        deps!();
        impl_lint_pass ! (MissingDoc => [MISSING_DOCS]) ;
    };
}

macro_29!();