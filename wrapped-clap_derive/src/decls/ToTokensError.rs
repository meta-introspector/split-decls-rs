macro_rules! ToTokensError {
    () => {
        pub (crate) trait ToTokensError { # [allow (non_snake_case)] fn EXPECTED_Span_OR_ToTokens < D : std :: fmt :: Display > (& self , msg : D) -> syn :: Error ; }
    };
}

ToTokensError!()