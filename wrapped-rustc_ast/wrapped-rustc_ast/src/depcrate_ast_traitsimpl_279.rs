// Generated macro for impl_279 (impl)
macro_rules! Depcrate_ast_traitsimpl_279 {
() => {
// Module: crate::ast_traits
// Provides: {"impl_279"}
// Dependencies: {}
impl HasTokens for Attribute { fn tokens (& self) -> Option < & LazyAttrTokenStream > { match & self . kind { AttrKind :: Normal (normal) => normal . tokens . as_ref () , kind @ AttrKind :: DocComment (..) => { panic ! ("Called tokens on doc comment attr {kind:?}") } } } fn tokens_mut (& mut self) -> Option < & mut Option < LazyAttrTokenStream > > { Some (match & mut self . kind { AttrKind :: Normal (normal) => & mut normal . tokens , kind @ AttrKind :: DocComment (..) => { panic ! ("Called tokens_mut on doc comment attr {kind:?}") } }) } }
};
}
