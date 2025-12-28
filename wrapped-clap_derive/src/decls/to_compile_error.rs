macro_rules! to_compile_error {
    () => {
        fn to_compile_error (error : syn :: Error , dummy : proc_macro2 :: TokenStream ,) -> proc_macro2 :: TokenStream { let compile_errors = error . to_compile_error () ; quote :: quote ! (# dummy # compile_errors) }
    };
}

to_compile_error!();