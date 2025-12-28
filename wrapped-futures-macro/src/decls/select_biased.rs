macro_rules! select_biased {
    () => {
        # [doc = " The `select_biased!` macro."] pub (crate) fn select_biased (input : TokenStream) -> TokenStream { select_inner (input , false) }
    };
}

select_biased!();