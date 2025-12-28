macro_rules! result_type {
    () => {
        fn result_type () -> TokenStream { # [cfg (feature = "std")] quote ! { :: std :: result :: Result } # [cfg (not (feature = "std"))] quote ! { :: core :: result :: Result } }
    };
}

result_type!();