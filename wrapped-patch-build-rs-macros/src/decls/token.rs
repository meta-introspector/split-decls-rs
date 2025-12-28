macro_rules! token {
    () => {
        # [proc_macro] pub fn token (input : TokenStream) -> TokenStream { solana_lift :: token_macro_impl (input) }
    };
}

token!()