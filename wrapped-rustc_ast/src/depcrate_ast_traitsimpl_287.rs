// Generated macro for impl_287 (impl)
macro_rules! Depcrate_ast_traitsimpl_287 {
() => {
// Module: crate::ast_traits
// Provides: {"impl_287"}
// Dependencies: {}
impl < T : HasAttrs > HasAttrs for Option < T > { const SUPPORTS_CUSTOM_INNER_ATTRS : bool = T :: SUPPORTS_CUSTOM_INNER_ATTRS ; fn attrs (& self) -> & [Attribute] { self . as_ref () . map (| inner | inner . attrs ()) . unwrap_or (& []) } fn visit_attrs (& mut self , f : impl FnOnce (& mut AttrVec)) { if let Some (inner) = self . as_mut () { inner . visit_attrs (f) ; } } }
};
}
