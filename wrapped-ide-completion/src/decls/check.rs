macro_rules! check {
    () => {
        pub (crate) fn check (# [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect) { let actual = completion_list (ra_fixture) ; expect . assert_eq (& actual) ; }
    };
}

check!();