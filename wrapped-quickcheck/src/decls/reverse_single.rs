macro_rules! deps {
    () => {
        TestResult!();
    };
}

macro_rules! reverse_single {
    () => {
        deps!();
        # [test] fn reverse_single () { fn prop (xs : Vec < usize >) -> TestResult { if xs . len () != 1 { TestResult :: discard () } else { TestResult :: from_bool (xs == xs . clone () . into_iter () . rev () . collect :: < Vec < _ > > () ,) } } quickcheck (prop as fn (Vec < usize >) -> TestResult) ; }
    };
}

reverse_single!();