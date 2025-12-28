macro_rules! include_dir {
    () => {
        # [proc_macro] pub fn include_dir (input : TokenStream) -> TokenStream { let foo = input . to_string () ; let args = parse_token_trees (& foo) . unwrap () ; let gen = impl_include_dir (args) . unwrap () ; gen . parse () . unwrap () }
    };
}

include_dir!()