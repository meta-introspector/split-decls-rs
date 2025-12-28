macro_rules! mkbuildrs {
    () => {
        # [proc_macro] pub fn mkbuildrs (input : TokenStream) -> TokenStream { mkbuildrs :: mkbuildrs_impl (input) }
    };
}

mkbuildrs!();