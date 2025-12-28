macro_rules! formal_verification {
    () => {
        # [proc_macro] # [decl2 (fn , name = "formal_verification" , vis = "pub" , hash = "7d168d21")] pub fn formal_verification (input : TokenStream) -> TokenStream { lean4_proof :: formal_verification_impl (input) }
    };
}

formal_verification!()