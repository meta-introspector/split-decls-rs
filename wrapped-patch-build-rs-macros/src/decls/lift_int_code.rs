macro_rules! lift_int_code {
    () => {
        # [proc_macro] # [decl2 (fn , name = "lift_int_code" , vis = "pub" , hash = "28039740")] pub fn lift_int_code (input : TokenStream) -> TokenStream { solana_lift :: lift_int_code_impl (input) }
    };
}

lift_int_code!()