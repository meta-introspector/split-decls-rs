macro_rules! real_eigenmatrix {
    () => {
        # [proc_macro] # [decl2 (fn , name = "real_eigenmatrix" , vis = "pub" , hash = "4e3fc25b")] pub fn real_eigenmatrix (input : TokenStream) -> TokenStream { real_data_analysis :: real_eigenmatrix_impl (input) }
    };
}

real_eigenmatrix!();