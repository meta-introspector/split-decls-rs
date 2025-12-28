macro_rules! option_type {
    () => {
        fn option_type () -> TokenStream { # [cfg (feature = "std")] quote ! { :: std :: option :: Option } # [cfg (not (feature = "std"))] quote ! { :: core :: option :: Option } }
    };
}

option_type!()