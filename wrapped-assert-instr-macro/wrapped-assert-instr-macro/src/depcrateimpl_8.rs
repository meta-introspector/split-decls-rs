// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl < T > quote :: ToTokens for Append < T > where T : Clone + IntoIterator , T :: Item : quote :: ToTokens , { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { for item in self . 0 . clone () { item . to_tokens (tokens) ; } } }
};
}
