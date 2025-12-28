macro_rules! compiler_inventory {
    () => {
        # [proc_macro] # [decl2 (fn , name = "compiler_inventory" , vis = "pub" , hash = "30a9d6d8")] pub fn compiler_inventory (input : TokenStream) -> TokenStream { compiler_inventory :: compiler_inventory_impl (input) }
    };
}

compiler_inventory!();