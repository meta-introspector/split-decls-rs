macro_rules! check_with_private_editable {
    () => {
        pub (crate) fn check_with_private_editable (# [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect ,) { let actual = completion_list_no_kw_with_private_editable (ra_fixture) ; expect . assert_eq (& actual) ; }
    };
}

check_with_private_editable!()