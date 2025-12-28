macro_rules! load_historical {
    () => {
        # [proc_macro] # [decl2 (fn , name = "load_historical" , vis = "pub" , hash = "96076756")] pub fn load_historical (input : TokenStream) -> TokenStream { quant_trading :: load_historical_impl (input) }
    };
}

load_historical!()