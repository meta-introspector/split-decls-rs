macro_rules! macro_42 {
    () => {
        declare_lint_pass ! (# [doc = " Checks for use of anonymous parameters (RFC 1685)."] AnonymousParameters => [ANONYMOUS_PARAMETERS]) ;
    };
}

macro_42!();