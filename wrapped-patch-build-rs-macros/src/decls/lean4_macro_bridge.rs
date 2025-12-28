macro_rules! lean4_macro_bridge {
    () => {
        # [proc_macro] # [decl2 (fn , name = "lean4_macro_bridge" , vis = "pub" , hash = "32d96c0a")] pub fn lean4_macro_bridge (input : TokenStream) -> TokenStream { lean4_mirror :: lean4_macro_bridge_impl (input) }
    };
}

lean4_macro_bridge!();