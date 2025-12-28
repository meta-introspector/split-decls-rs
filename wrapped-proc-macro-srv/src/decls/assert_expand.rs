macro_rules! assert_expand {
    () => {
        pub fn assert_expand (macro_name : & str , # [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect , expect_spanned : Expect ,) { assert_expand_impl (macro_name , ra_fixture , None , expect , expect_spanned) ; }
    };
}

assert_expand!()