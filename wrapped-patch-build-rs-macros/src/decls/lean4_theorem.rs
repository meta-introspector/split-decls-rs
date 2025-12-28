macro_rules! lean4_theorem {
    () => {
        # [proc_macro] # [decl2 (fn , name = "lean4_theorem" , vis = "pub" , hash = "d26355c2")] pub fn lean4_theorem (input : TokenStream) -> TokenStream { lean4_proof :: lean4_theorem_impl (input) }
    };
}

lean4_theorem!();