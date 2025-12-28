macro_rules! grast_structural {
    () => {
        # [proc_macro] # [decl2 (fn , name = "grast_structural" , vis = "pub" , hash = "aa982462")] pub fn grast_structural (input : TokenStream) -> TokenStream { duplicate_analysis :: grast_structural_impl (input) }
    };
}

grast_structural!();