// Generated macro for impl_40 (impl)
macro_rules! Depcrate_astimpl_40 {
() => {
// Module: crate::ast
// Provides: {"impl_40"}
// Dependencies: {}
impl ToTokens for Params { fn to_tokens (& self , tokens : & mut TokenStream) { NestedTuple (self . 0 . as_slice ()) . to_tokens (tokens) } }
};
}
