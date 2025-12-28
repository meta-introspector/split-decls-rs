macro_rules! proof_simulate {
    () => {
        # [proc_macro] # [decl2 (fn , name = "proof_simulate" , vis = "pub" , hash = "7d3e1840")] pub fn proof_simulate (input : TokenStream) -> TokenStream { lean4_mirror :: proof_simulate_impl (input) }
    };
}

proof_simulate!();