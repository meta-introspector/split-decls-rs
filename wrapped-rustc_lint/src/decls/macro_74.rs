macro_rules! macro_74 {
    () => {
        declare_lint_pass ! (# [doc = " Lint for expressions of the form `--x` that can be confused with C's"] # [doc = " prefix decrement operator."] DoubleNegations => [DOUBLE_NEGATIONS]) ;
    };
}

macro_74!()