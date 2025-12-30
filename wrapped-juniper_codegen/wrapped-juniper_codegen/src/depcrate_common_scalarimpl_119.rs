// Generated macro for impl_119 (impl)
macro_rules! Depcrate_common_scalarimpl_119 {
() => {
// Module: crate::common::scalar
// Provides: {"impl_119"}
// Dependencies: {}
impl ToTokens for AttrValue { fn to_tokens (& self , into : & mut TokenStream) { match self { Self :: Concrete (ty) => ty . to_tokens (into) , Self :: Generic (pred) => pred . to_tokens (into) , } } }
};
}
