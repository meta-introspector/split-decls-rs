macro_rules! qc_tests {
    () => {
        fn qc_tests () -> u64 { let default = 100 ; match env :: var ("QUICKCHECK_TESTS") { Ok (val) => val . parse () . unwrap_or (default) , Err (_) => default , } }
    };
}

qc_tests!()