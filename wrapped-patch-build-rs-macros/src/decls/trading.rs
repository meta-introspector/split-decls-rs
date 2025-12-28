macro_rules! trading {
    () => {
        # [proc_macro] # [decl2 (fn , name = "trading" , vis = "pub" , hash = "df75a15c")] pub fn trading (input : TokenStream) -> TokenStream { quant_trading :: trading_impl (input) }
    };
}

trading!();