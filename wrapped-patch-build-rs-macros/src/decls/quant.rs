macro_rules! quant {
    () => {
        # [proc_macro] # [decl2 (fn , name = "quant" , vis = "pub" , hash = "47276e9c")] pub fn quant (input : TokenStream) -> TokenStream { quant_trading :: quant_impl (input) }
    };
}

quant!();