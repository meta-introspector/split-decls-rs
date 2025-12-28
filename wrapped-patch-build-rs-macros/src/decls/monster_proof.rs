macro_rules! monster_proof {
    () => {
        # [proc_macro] pub fn monster_proof (input : TokenStream) -> TokenStream { lean4_proof :: monster_proof_impl (input) }
    };
}

monster_proof!()