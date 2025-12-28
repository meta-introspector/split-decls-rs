macro_rules! box_type {
    () => {
        fn box_type () -> TokenStream { # [cfg (feature = "std")] quote ! { :: std :: boxed :: Box } # [cfg (not (feature = "std"))] quote ! { :: alloc :: boxed :: Box } }
    };
}

box_type!();