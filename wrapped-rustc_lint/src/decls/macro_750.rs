macro_rules! macro_750 {
    () => {
        declare_lint_pass ! (PtrNullChecks => [USELESS_PTR_NULL_CHECKS , INVALID_NULL_ARGUMENTS]) ;
    };
}

macro_750!();