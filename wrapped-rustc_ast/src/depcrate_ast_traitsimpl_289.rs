// Generated macro for impl_289 (impl)
macro_rules! Depcrate_ast_traitsimpl_289 {
() => {
// Module: crate::ast_traits
// Provides: {"impl_289"}
// Dependencies: {}
impl HasAttrs for Stmt { const SUPPORTS_CUSTOM_INNER_ATTRS : bool = StmtKind :: SUPPORTS_CUSTOM_INNER_ATTRS ; fn attrs (& self) -> & [Attribute] { self . kind . attrs () } fn visit_attrs (& mut self , f : impl FnOnce (& mut AttrVec)) { self . kind . visit_attrs (f) ; } }
};
}
