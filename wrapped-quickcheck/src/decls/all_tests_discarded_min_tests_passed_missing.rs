macro_rules! deps {
    () => {
        QuickCheck!();
        TestResult!();
    };
}

macro_rules! all_tests_discarded_min_tests_passed_missing {
    () => {
        deps!();
        # [test] fn all_tests_discarded_min_tests_passed_missing () { fn prop_discarded (_ : u8) -> TestResult { TestResult :: discard () } QuickCheck :: new () . quickcheck (prop_discarded as fn (u8) -> TestResult) ; }
    };
}

all_tests_discarded_min_tests_passed_missing!();