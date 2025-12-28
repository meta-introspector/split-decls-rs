macro_rules! deps {
    () => {
        Gen!();
        QuickCheck!();
    };
}

macro_rules! regression_issue_83 {
    () => {
        deps!();
        # [test] fn regression_issue_83 () { fn prop (_ : u8) -> bool { true } QuickCheck :: new () . set_rng (Gen :: new (1024)) . quickcheck (prop as fn (u8) -> bool) ; }
    };
}

regression_issue_83!()