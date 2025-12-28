macro_rules! workspace_members_list_impl {
    () => {
        pub fn workspace_members_list_impl (input : TokenStream) -> TokenStream { let parsed : Punctuated < LitStr , Token ! [,] > = parse_macro_input ! (input with Punctuated :: parse_terminated) ; let expanded = quote ! { [# parsed] } ; expanded . into () }
    };
}

workspace_members_list_impl!();