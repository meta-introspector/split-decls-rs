macro_rules! is_nightly {
    () => {
        pub fn is_nightly () -> bool { let vv = & rustc_info () . verbose_version ; env :: var ("CARGO_TEST_DISABLE_NIGHTLY") . is_err () && (vv . contains ("-nightly") || vv . contains ("-dev")) }
    };
}

is_nightly!()