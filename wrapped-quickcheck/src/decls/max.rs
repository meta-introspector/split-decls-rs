macro_rules! deps {
    () => {
        TestResult!();
    };
}

macro_rules! max {
    () => {
        deps!();
        # [test] fn max () { fn prop (x : isize , y : isize) -> TestResult { if x > y { TestResult :: discard () } else { TestResult :: from_bool (:: std :: cmp :: max (x , y) == y) } } quickcheck (prop as fn (isize , isize) -> TestResult) ; }
    };
}

max!()