macro_rules! do_double {
    () => {
        fn do_double (_attrs : TokenStream , input : TokenStream) -> TokenStream { let mut item : Item = match parse2 (input . clone ()) { Ok (u) => u , Err (e) => return e . to_compile_error () } ; match & mut item { Item :: Use (use_stmt) => mock_itemuse (use_stmt) , Item :: Type (item_type) => mock_itemtype (item_type) , _ => { compile_error (item . span () , "Only use statements and type aliases may be doubled") ; } } ; quote ! (# [cfg (not (test))] # input # [cfg (test)] # item) }
    };
}

do_double!();