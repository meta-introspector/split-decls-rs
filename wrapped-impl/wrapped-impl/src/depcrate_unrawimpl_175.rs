// Generated macro for impl_175 (impl)
macro_rules! Depcrate_unrawimpl_175 {
() => {
// Module: crate::unraw
// Provides: {"impl_175"}
// Dependencies: {}
impl ToTokens for MemberUnraw { fn to_tokens (& self , tokens : & mut TokenStream) { match self { MemberUnraw :: Named (ident) => ident . to_local () . to_tokens (tokens) , MemberUnraw :: Unnamed (index) => index . to_tokens (tokens) , } } }
};
}
