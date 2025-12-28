macro_rules! qc_max_tests {
    () => {
        fn qc_max_tests () -> u64 { let default = 10_000 ; match env :: var ("QUICKCHECK_MAX_TESTS") { Ok (val) => val . parse () . unwrap_or (default) , Err (_) => default , } }
    };
}

qc_max_tests!();