macro_rules! assert_expand_attr {
    () => {
        pub fn assert_expand_attr (macro_name : & str , # [rust_analyzer :: rust_fixture] ra_fixture : & str , attr_args : & str , expect : Expect , expect_spanned : Expect ,) { assert_expand_impl (macro_name , ra_fixture , Some (attr_args) , expect , expect_spanned) ; }
    };
}

assert_expand_attr!();