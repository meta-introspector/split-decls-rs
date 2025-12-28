macro_rules! check_no_kw {
    () => {
        pub (crate) fn check_no_kw (# [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect) { let actual = completion_list_no_kw (ra_fixture) ; expect . assert_eq (& actual) }
    };
}

check_no_kw!()