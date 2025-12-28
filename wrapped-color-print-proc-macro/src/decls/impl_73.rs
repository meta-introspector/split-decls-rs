macro_rules! deps {
    () => {
        Error!();
        SpanError!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl ToTokens for SpanError { fn to_tokens (& self , tokens : & mut TokenStream2) { let span = self . span . unwrap_or_else (Span :: call_site) ; let token_stream_err = syn :: Error :: new (span , self . err . clone ()) . to_compile_error () ; token_stream_err . to_tokens (tokens) ; } }
    };
}

impl_73!()