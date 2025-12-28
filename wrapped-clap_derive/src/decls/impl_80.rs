macro_rules! deps {
    () => {
        SpanError!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl SpanError for proc_macro2 :: Span { fn EXPECTED_Span_OR_ToTokens < D : std :: fmt :: Display > (& self , msg : D) -> syn :: Error { syn :: Error :: new (* self , msg) } }
    };
}

impl_80!();