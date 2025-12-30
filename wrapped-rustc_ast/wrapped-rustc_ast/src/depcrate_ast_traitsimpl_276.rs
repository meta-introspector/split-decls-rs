// Generated macro for impl_276 (impl)
macro_rules! Depcrate_ast_traitsimpl_276 {
() => {
// Module: crate::ast_traits
// Provides: {"impl_276"}
// Dependencies: {}
impl < T : HasTokens > HasTokens for Box < T > { fn tokens (& self) -> Option < & LazyAttrTokenStream > { (* * self) . tokens () } fn tokens_mut (& mut self) -> Option < & mut Option < LazyAttrTokenStream > > { (* * self) . tokens_mut () } }
};
}
