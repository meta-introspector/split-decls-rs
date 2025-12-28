macro_rules! dep_version {
    () => {
        # [proc_macro] pub fn dep_version (input : TokenStream) -> TokenStream { macros :: dep_version_impl (input) }
    };
}

dep_version!();