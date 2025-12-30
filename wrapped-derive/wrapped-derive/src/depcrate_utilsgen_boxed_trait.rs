// Generated macro for gen_boxed_trait (function)
macro_rules! Depcrate_utilsgen_boxed_trait {
() => {
// Module: crate::utils
// Provides: {"gen_boxed_trait"}
// Dependencies: {}
pub fn gen_boxed_trait (crate_name : & TokenStream) -> TokenStream { if cfg ! (feature = "boxed-trait") { quote ! { # [# crate_name :: async_trait :: async_trait] } } else { quote ! { } } }
};
}
