macro_rules! gen_boxed_trait {
    () => {
        pub fn gen_boxed_trait (crate_name : & TokenStream) -> TokenStream { if cfg ! (feature = "boxed-trait") { quote ! { # [# crate_name :: async_trait :: async_trait] } } else { quote ! { } } }
    };
}

gen_boxed_trait!();