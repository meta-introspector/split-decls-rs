macro_rules! CARGO_INTEGRATION_TEST_DIR {
    () => {
        static CARGO_INTEGRATION_TEST_DIR : & str = "cit" ;
    };
}

CARGO_INTEGRATION_TEST_DIR!();