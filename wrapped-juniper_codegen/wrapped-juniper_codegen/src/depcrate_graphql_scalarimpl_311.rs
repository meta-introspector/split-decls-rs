// Generated macro for impl_311 (impl)
macro_rules! Depcrate_graphql_scalarimpl_311 {
() => {
// Module: crate::graphql_scalar
// Provides: {"impl_311"}
// Dependencies: {}
impl ToTokens for Field { fn to_tokens (& self , tokens : & mut TokenStream) { match self { Self :: Named (f) => f . ident . to_tokens (tokens) , Self :: Unnamed (_) => tokens . append (Literal :: u8_unsuffixed (0)) , } } }
};
}
