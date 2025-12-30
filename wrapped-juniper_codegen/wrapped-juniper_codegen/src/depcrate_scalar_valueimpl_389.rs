// Generated macro for impl_389 (impl)
macro_rules! Depcrate_scalar_valueimpl_389 {
() => {
// Module: crate::scalar_value
// Provides: {"impl_389"}
// Dependencies: {}
impl ToTokens for Field { fn to_tokens (& self , tokens : & mut TokenStream) { match self { Self :: Named (f) => f . ident . to_tokens (tokens) , Self :: Unnamed => tokens . append (Literal :: u8_unsuffixed (0)) , } } }
};
}
