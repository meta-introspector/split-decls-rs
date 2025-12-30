// Generated macro for impl_21 (impl)
macro_rules! Depcrate_attrimpl_21 {
() => {
// Module: crate::attr
// Provides: {"impl_21"}
// Dependencies: {}
impl ToTokens for AttrValue { fn to_tokens (& self , tokens : & mut TokenStream) { match self { Self :: LitStr (t) => t . to_tokens (tokens) , Self :: Expr (t) => t . to_tokens (tokens) , Self :: Call (t) => { let t = quote ! (# (# t) ,*) ; t . to_tokens (tokens) ; } } } }
};
}
