// Generated macro for impl_81 (impl)
macro_rules! Depcrate_initializerimpl_81 {
() => {
// Module: crate::initializer
// Provides: {"impl_81"}
// Dependencies: {}
impl ToTokens for MatchSome < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { match * self { Self :: Move => tokens . append_all (quote ! (Some (value) => value)) , Self :: Clone { crate_root } => tokens . append_all (quote ! (Some (ref value) => # crate_root :: export :: core :: clone :: Clone :: clone (value))) , } } }
};
}
