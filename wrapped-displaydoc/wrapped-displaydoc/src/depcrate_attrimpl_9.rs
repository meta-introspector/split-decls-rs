// Generated macro for impl_9 (impl)
macro_rules! Depcrate_attrimpl_9 {
() => {
// Module: crate::attr
// Provides: {"impl_9"}
// Dependencies: {}
impl ToTokens for VariantDisplay { fn to_tokens (& self , tokens : & mut TokenStream) { if let Some (ref r#enum) = self . r#enum { r#enum . to_tokens (tokens) ; tokens . extend (quote ! { ?; write ! (formatter , ": ") ?; }) ; } self . variant . to_tokens (tokens) ; } }
};
}
