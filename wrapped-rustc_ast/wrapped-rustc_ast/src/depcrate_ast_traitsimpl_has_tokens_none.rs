// Generated macro for impl_has_tokens_none (macro)
macro_rules! Depcrate_ast_traitsimpl_has_tokens_none {
() => {
// Module: crate::ast_traits
// Provides: {"impl_has_tokens_none"}
// Dependencies: {}
macro_rules ! impl_has_tokens_none { ($ ($ T : ty) ,+ $ (,) ?) => { $ (impl HasTokens for $ T { fn tokens (& self) -> Option <& LazyAttrTokenStream > { None } fn tokens_mut (& mut self) -> Option <& mut Option < LazyAttrTokenStream >> { None } }) + } ; }
};
}
