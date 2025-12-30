// Generated macro for impl_8 (impl)
macro_rules! Depcrate_attrimpl_8 {
() => {
// Module: crate::attr
// Provides: {"impl_8"}
// Dependencies: {}
impl ToTokens for Display { fn to_tokens (& self , tokens : & mut TokenStream) { let fmt = & self . fmt ; let args = & self . args ; tokens . extend (quote ! { write ! (formatter , # fmt # args) }) ; } }
};
}
