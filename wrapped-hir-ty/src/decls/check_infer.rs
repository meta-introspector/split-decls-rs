macro_rules! check_infer {
    () => {
        fn check_infer (# [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect) { let mut actual = infer (ra_fixture) ; actual . push ('\n') ; expect . assert_eq (& actual) ; }
    };
}

check_infer!()