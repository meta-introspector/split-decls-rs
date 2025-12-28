macro_rules! deps {
    () => {
        QuickCheck!();
        Gen!();
    };
}

macro_rules! regression_issue_83_signed {
    () => {
        deps!();
        # [test] fn regression_issue_83_signed () { fn prop (_ : i8) -> bool { true } QuickCheck :: new () . set_rng (Gen :: new (1024)) . quickcheck (prop as fn (i8) -> bool) ; }
    };
}

regression_issue_83_signed!();