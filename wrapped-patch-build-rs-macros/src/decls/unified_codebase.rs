macro_rules! unified_codebase {
    () => {
        # [proc_macro] # [decl2 (fn , name = "unified_codebase" , vis = "pub" , hash = "70bfd79e")] pub fn unified_codebase (input : TokenStream) -> TokenStream { duplicate_analysis :: unified_codebase_impl (input) }
    };
}

unified_codebase!()