// Generated macro for impl_68 (impl)
macro_rules! Depcrate_astimpl_68 {
() => {
// Module: crate::ast
// Provides: {"impl_68"}
// Dependencies: {}
impl < 'a > ToTokens for FreshVar < 'a > { fn to_tokens (& self , tokens : & mut TokenStream) { let ident = format ! ("{}_{}" , self . prefix , self . count) ; call_site_ident (& ident) . to_tokens (tokens) } }
};
}
