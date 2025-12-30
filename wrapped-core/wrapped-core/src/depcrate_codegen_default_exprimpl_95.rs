// Generated macro for impl_95 (impl)
macro_rules! Depcrate_codegen_default_exprimpl_95 {
() => {
// Module: crate::codegen::default_expr
// Provides: {"impl_95"}
// Dependencies: {}
impl ToTokens for DefaultExpression < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append_all (match * self { DefaultExpression :: Inherit (ident) => { let dsn = Ident :: new (DEFAULT_STRUCT_NAME , :: proc_macro2 :: Span :: call_site ()) ; quote ! (# dsn .# ident) } DefaultExpression :: Explicit (callable) => { quote_spanned ! (callable . span () => :: darling :: export :: identity ::< fn () -> _ > (# callable) ()) } DefaultExpression :: Trait { span } => { quote_spanned ! (span => :: darling :: export :: Default :: default ()) } }) ; } }
};
}
