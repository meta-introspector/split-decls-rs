macro_rules! join_internal {
    () => {
        # [doc = " The `join!` macro."] # [proc_macro] pub fn join_internal (input : TokenStream) -> TokenStream { crate :: join :: join (input) }
    };
}

join_internal!();