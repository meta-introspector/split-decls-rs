macro_rules! workspace_members_list {
    () => {
        # [proc_macro] pub fn workspace_members_list (input : TokenStream) -> TokenStream { macros :: workspace_members_list_impl (input) }
    };
}

workspace_members_list!();