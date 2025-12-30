// Generated macro for impl_286 (impl)
macro_rules! Depcrate_ast_traitsimpl_286 {
() => {
// Module: crate::ast_traits
// Provides: {"impl_286"}
// Dependencies: {}
impl < T : HasAttrs > HasAttrs for Box < T > { const SUPPORTS_CUSTOM_INNER_ATTRS : bool = T :: SUPPORTS_CUSTOM_INNER_ATTRS ; fn attrs (& self) -> & [Attribute] { (* * self) . attrs () } fn visit_attrs (& mut self , f : impl FnOnce (& mut AttrVec)) { (* * self) . visit_attrs (f) ; } }
};
}
