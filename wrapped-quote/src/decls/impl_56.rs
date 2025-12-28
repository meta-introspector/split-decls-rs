macro_rules! deps {
    () => {
        ToTokens!();
        Spanned!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < T : ? Sized + ToTokens > Spanned for T { fn __span (& self) -> Span { join_spans (self . into_token_stream ()) } }
    };
}

impl_56!();