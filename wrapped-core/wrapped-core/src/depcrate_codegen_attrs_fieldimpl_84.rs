// Generated macro for impl_84 (impl)
macro_rules! Depcrate_codegen_attrs_fieldimpl_84 {
() => {
// Module: crate::codegen::attrs_field
// Provides: {"impl_84"}
// Dependencies: {}
impl ToTokens for ValuePopulator < '_ > { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { let ForwardedField { ident , with } = self . 0 ; let initializer_expr = match with { Some (with) => quote_spanned ! (with . span () => __errors . handle (# with (__fwd_attrs))) , None => quote ! (:: darling :: export :: Some (__fwd_attrs)) , } ; tokens . append_all (quote ! (# ident = # initializer_expr ;)) ; } }
};
}
