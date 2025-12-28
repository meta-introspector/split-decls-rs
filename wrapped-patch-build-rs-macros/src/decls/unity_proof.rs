macro_rules! unity_proof {
    () => {
        # [proc_macro] # [decl2 (fn , name = "unity_proof" , vis = "pub" , hash = "7c9d4384")] pub fn unity_proof (input : TokenStream) -> TokenStream { sat_lfunction :: unity_proof_impl (input) }
    };
}

unity_proof!()