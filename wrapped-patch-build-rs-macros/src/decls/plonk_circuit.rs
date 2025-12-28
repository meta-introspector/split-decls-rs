macro_rules! plonk_circuit {
    () => {
        # [proc_macro] # [decl2 (fn , name = "plonk_circuit" , vis = "pub" , hash = "473ff43a")] pub fn plonk_circuit (input : TokenStream) -> TokenStream { zk_proof :: plonk_circuit_impl (input) }
    };
}

plonk_circuit!();