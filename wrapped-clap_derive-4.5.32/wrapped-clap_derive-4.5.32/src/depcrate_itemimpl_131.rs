// Generated macro for impl_131 (impl)
macro_rules! Depcrate_itemimpl_131 {
() => {
// Module: crate::item
// Provides: {"impl_131"}
// Dependencies: {}
impl ToTokens for Name { fn to_tokens (& self , tokens : & mut TokenStream) { match self { Name :: Assigned (t) => t . to_tokens (tokens) , Name :: Derived (ident) => { let s = ident . unraw () . to_string () ; quote_spanned ! (ident . span () => # s) . to_tokens (tokens) ; } } } }
};
}
