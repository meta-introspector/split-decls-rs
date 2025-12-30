// Generated macro for impl_294 (impl)
macro_rules! Depcrate_ast_traitsimpl_294 {
() => {
// Module: crate::ast_traits
// Provides: {"impl_294"}
// Dependencies: {}
impl < Wrapped : HasAttrs , Tag > HasAttrs for AstNodeWrapper < Wrapped , Tag > { const SUPPORTS_CUSTOM_INNER_ATTRS : bool = Wrapped :: SUPPORTS_CUSTOM_INNER_ATTRS ; fn attrs (& self) -> & [Attribute] { self . wrapped . attrs () } fn visit_attrs (& mut self , f : impl FnOnce (& mut AttrVec)) { self . wrapped . visit_attrs (f) ; } }
};
}
