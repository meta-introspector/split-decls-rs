macro_rules! deps {
    () => {
        ToTokensError!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < T : quote :: ToTokens > ToTokensError for T { fn EXPECTED_Span_OR_ToTokens < D : std :: fmt :: Display > (& self , msg : D) -> syn :: Error { syn :: Error :: new_spanned (self . to_token_stream () , msg) } }
    };
}

impl_79!()