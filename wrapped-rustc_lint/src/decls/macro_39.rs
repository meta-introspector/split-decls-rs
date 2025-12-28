macro_rules! deps {
    () => {
        MissingDebugImplementations!();
    };
}

macro_rules! macro_39 {
    () => {
        deps!();
        impl_lint_pass ! (MissingDebugImplementations => [MISSING_DEBUG_IMPLEMENTATIONS]) ;
    };
}

macro_39!();