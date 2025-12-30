// Generated macro for gen_deprecation (function)
macro_rules! Depcrate_utilsgen_deprecation {
() => {
// Module: crate::utils
// Provides: {"gen_deprecation"}
// Dependencies: {}
pub fn gen_deprecation (deprecation : & Deprecation , crate_name : & TokenStream) -> TokenStream { match deprecation { Deprecation :: NoDeprecated => { quote ! { # crate_name :: registry :: Deprecation :: NoDeprecated } } Deprecation :: Deprecated { reason : Some (reason) , } => { quote ! { # crate_name :: registry :: Deprecation :: Deprecated { reason : :: std :: option :: Option :: Some (:: std :: string :: ToString :: to_string (# reason)) } } } Deprecation :: Deprecated { reason : None } => { quote ! { # crate_name :: registry :: Deprecation :: Deprecated { reason : :: std :: option :: Option :: None } } } } }
};
}
