macro_rules! purchase_blocks {
    () => {
        # [proc_macro] # [decl2 (fn , name = "purchase_blocks" , vis = "pub" , hash = "bb48e11f")] pub fn purchase_blocks (input : TokenStream) -> TokenStream { solana_lift :: purchase_blocks_impl (input) }
    };
}

purchase_blocks!()