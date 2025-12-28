macro_rules! extract_lfunction {
    () => {
        # [proc_macro] # [decl2 (fn , name = "extract_lfunction" , vis = "pub" , hash = "0bd53dc8")] pub fn extract_lfunction (input : TokenStream) -> TokenStream { sat_lfunction :: extract_lfunction_impl (input) }
    };
}

extract_lfunction!();