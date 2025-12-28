macro_rules! test_internal {
    () => {
        # [proc_macro_attribute] pub fn test_internal (input : TokenStream , item : TokenStream) -> TokenStream { crate :: executor :: test (input , item) }
    };
}

test_internal!()