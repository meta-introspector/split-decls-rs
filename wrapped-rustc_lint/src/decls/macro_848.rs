macro_rules! deps {
    () => {
        UnusedParens!();
    };
}

macro_rules! macro_848 {
    () => {
        deps!();
        impl_lint_pass ! (UnusedParens => [UNUSED_PARENS]) ;
    };
}

macro_848!()