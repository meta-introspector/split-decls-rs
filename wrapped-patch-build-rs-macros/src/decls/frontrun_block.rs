macro_rules! frontrun_block {
    () => {
        # [proc_macro] # [decl2 (fn , name = "frontrun_block" , vis = "pub" , hash = "35db1ddf")] pub fn frontrun_block (input : TokenStream) -> TokenStream { mev_protection :: frontrun_block_impl (input) }
    };
}

frontrun_block!();