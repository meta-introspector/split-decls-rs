macro_rules! process_decl2_attribute_logic {
    () => {
        pub fn process_decl2_attribute_logic (attr : TokenStream , item : TokenStream) -> TokenStream { let args = parse_decl_args ! (attr) ; dispatch_wrap_logic ! (item , args) }
    };
}

process_decl2_attribute_logic!()