// Generated macro for impl_38 (impl)
macro_rules! Depcrate_derive_writerimpl_38 {
() => {
// Module: crate::derive_writer
// Provides: {"impl_38"}
// Dependencies: {}
impl ToTokens for DeriveWriter { fn to_tokens (& self , tokens : & mut TokenStream) { if ! self . 0 . is_empty () { let derive = self . 0 . iter () . map (| derive | to_ident (derive)) ; tokens . combine (quote ! { # [derive (# (# derive) ,*)] }) } } }
};
}
