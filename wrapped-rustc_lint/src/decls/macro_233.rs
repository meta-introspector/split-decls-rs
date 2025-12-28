macro_rules! deps {
    () => {
        IfLetRescope!();
    };
}

macro_rules! macro_233 {
    () => {
        deps!();
        impl_lint_pass ! (IfLetRescope => [IF_LET_RESCOPE]) ;
    };
}

macro_233!()