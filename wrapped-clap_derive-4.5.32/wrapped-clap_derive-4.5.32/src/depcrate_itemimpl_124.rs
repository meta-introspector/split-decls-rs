// Generated macro for impl_124 (impl)
macro_rules! Depcrate_itemimpl_124 {
() => {
// Module: crate::item
// Provides: {"impl_124"}
// Dependencies: {}
impl ToTokens for Deprecation { fn to_tokens (& self , ts : & mut TokenStream) { let tokens = if cfg ! (feature = "deprecated") { let Deprecation { span , id , version , description , } = self ; let span = * span ; let id = Ident :: new (id , span) ; quote_spanned ! (span => { # [deprecated (since = # version , note = # description)] fn # id () { } # id () ; }) } else { quote ! () } ; tokens . to_tokens (ts) ; } }
};
}
