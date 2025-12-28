macro_rules! matrix_decompose {
    () => {
        # [proc_macro] # [decl2 (fn , name = "matrix_decompose" , vis = "pub" , hash = "dc5acbf3")] pub fn matrix_decompose (input : TokenStream) -> TokenStream { sat_lfunction :: matrix_decompose_impl (input) }
    };
}

matrix_decompose!()