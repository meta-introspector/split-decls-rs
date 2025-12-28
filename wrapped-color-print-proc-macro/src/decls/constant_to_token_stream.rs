macro_rules! constant_to_token_stream {
    () => {
        # [doc = " Creates a new terminfo constant (available in the `color-print` package) as a token stream."] fn constant_to_token_stream (constant : & str) -> TokenStream2 { let constant_ident = util :: ident (constant) ; (quote ! { * color_print ::# constant_ident }) . into () }
    };
}

constant_to_token_stream!()