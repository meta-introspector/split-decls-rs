// Generated macro for impl_121 (impl)
macro_rules! Depcrate_itemimpl_121 {
() => {
// Module: crate::item
// Provides: {"impl_121"}
// Dependencies: {}
impl ToTokens for Method { fn to_tokens (& self , ts : & mut TokenStream) { let Method { ref name , ref args } = self ; let tokens = quote ! (.# name (# args)) ; tokens . to_tokens (ts) ; } }
};
}
