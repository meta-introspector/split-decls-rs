macro_rules! prove_eigenvalues {
    () => {
        # [proc_macro] # [decl2 (fn , name = "value" , vis = "pub" , hash = "5299a79b")] pub fn prove_eigenvalues (input : TokenStream) -> TokenStream { real_rustc_analysis :: prove_eigenvalues_impl (input) }
    };
}

prove_eigenvalues!()