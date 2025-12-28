macro_rules! rust_to_lean4 {
    () => {
        # [proc_macro] # [decl2 (fn , name = "rust_to_lean4" , vis = "pub" , hash = "a0aab282")] pub fn rust_to_lean4 (input : TokenStream) -> TokenStream { lean4_mirror :: rust_to_lean4_impl (input) }
    };
}

rust_to_lean4!()