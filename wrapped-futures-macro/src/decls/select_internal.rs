macro_rules! select_internal {
    () => {
        # [doc = " The `select!` macro."] # [proc_macro] pub fn select_internal (input : TokenStream) -> TokenStream { crate :: select :: select (input) }
    };
}

select_internal!();