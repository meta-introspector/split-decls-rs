macro_rules! deps {
    () => {
        EllipsisInclusiveRangePatterns!();
    };
}

macro_rules! macro_79 {
    () => {
        deps!();
        impl_lint_pass ! (EllipsisInclusiveRangePatterns => [ELLIPSIS_INCLUSIVE_RANGE_PATTERNS]) ;
    };
}

macro_79!();