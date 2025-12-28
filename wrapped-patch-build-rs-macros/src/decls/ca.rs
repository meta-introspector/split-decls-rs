macro_rules! ca {
    () => {
        # [proc_macro] pub fn ca (input : TokenStream) -> TokenStream { solana_lift :: ca_macro_impl (input) }
    };
}

ca!();