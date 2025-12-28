macro_rules! check_infer_with_mismatches {
    () => {
        fn check_infer_with_mismatches (# [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect) { let mut actual = infer_with_mismatches (ra_fixture , true) ; actual . push ('\n') ; expect . assert_eq (& actual) ; }
    };
}

check_infer_with_mismatches!();