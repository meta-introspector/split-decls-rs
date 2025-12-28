macro_rules! deps {
    () => {
        NonLocalDefinitions!();
    };
}

macro_rules! macro_681 {
    () => {
        deps!();
        impl_lint_pass ! (NonLocalDefinitions => [NON_LOCAL_DEFINITIONS]) ;
    };
}

macro_681!()