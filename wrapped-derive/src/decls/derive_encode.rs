macro_rules! derive_encode {
    () => {
        # [proc_macro_derive (Encode , attributes (bincode))] pub fn derive_encode (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { derive_encode_inner (input) . unwrap_or_else (| e | e . into_token_stream ()) }
    };
}

derive_encode!();