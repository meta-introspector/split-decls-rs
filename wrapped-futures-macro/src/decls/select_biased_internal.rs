macro_rules! select_biased_internal {
    () => {
        # [doc = " The `select_biased!` macro."] # [proc_macro] pub fn select_biased_internal (input : TokenStream) -> TokenStream { crate :: select :: select_biased (input) }
    };
}

select_biased_internal!();