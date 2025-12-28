macro_rules! eigenform_verify {
    () => {
        # [proc_macro] # [decl2 (fn , name = "eigenform_verify" , vis = "pub" , hash = "7ab3142e")] pub fn eigenform_verify (input : TokenStream) -> TokenStream { rust_eigenmatrix :: eigenform_verify_impl (input) }
    };
}

eigenform_verify!();