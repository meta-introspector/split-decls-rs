macro_rules! generate_checked_macros {
    () => {
        # [proc_macro] # [decl2 (fn , name = "generate_checked_macros" , vis = "pub" , hash = "fa86b97d")] pub fn generate_checked_macros (input : TokenStream) -> TokenStream { template_checker :: generate_checked_macros_impl (input) }
    };
}

generate_checked_macros!();