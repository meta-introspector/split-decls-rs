// Generated macro for impl_97 (impl)
macro_rules! Depcrate_codegen_default_exprimpl_97 {
() => {
// Module: crate::codegen::default_expr
// Provides: {"impl_97"}
// Dependencies: {}
impl ToTokens for DefaultDeclaration < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { let name = Ident :: new (DEFAULT_STRUCT_NAME , :: proc_macro2 :: Span :: call_site ()) ; let expr = self . 0 ; tokens . append_all (quote ! (let # name : Self = # expr ;)) ; } }
};
}
