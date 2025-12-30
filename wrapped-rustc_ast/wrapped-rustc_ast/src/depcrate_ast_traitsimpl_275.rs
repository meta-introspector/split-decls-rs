// Generated macro for impl_275 (impl)
macro_rules! Depcrate_ast_traitsimpl_275 {
() => {
// Module: crate::ast_traits
// Provides: {"impl_275"}
// Dependencies: {}
impl < T : HasTokens > HasTokens for Option < T > { fn tokens (& self) -> Option < & LazyAttrTokenStream > { self . as_ref () . and_then (| inner | inner . tokens ()) } fn tokens_mut (& mut self) -> Option < & mut Option < LazyAttrTokenStream > > { self . as_mut () . and_then (| inner | inner . tokens_mut ()) } }
};
}
