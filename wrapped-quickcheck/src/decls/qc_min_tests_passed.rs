macro_rules! qc_min_tests_passed {
    () => {
        fn qc_min_tests_passed () -> u64 { let default = 0 ; match env :: var ("QUICKCHECK_MIN_TESTS_PASSED") { Ok (val) => val . parse () . unwrap_or (default) , Err (_) => default , } }
    };
}

qc_min_tests_passed!();