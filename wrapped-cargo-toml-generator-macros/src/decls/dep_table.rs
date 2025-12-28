macro_rules! dep_table {
    () => {
        # [proc_macro] pub fn dep_table (input : TokenStream) -> TokenStream { macros :: dep_table_impl (input) }
    };
}

dep_table!();