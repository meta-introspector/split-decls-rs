// Generated macro for impl_has_attrs_none (macro)
macro_rules! Depcrate_ast_traitsimpl_has_attrs_none {
() => {
// Module: crate::ast_traits
// Provides: {"impl_has_attrs_none"}
// Dependencies: {}
macro_rules ! impl_has_attrs_none { ($ ($ T : ty) ,+ $ (,) ?) => { $ (impl HasAttrs for $ T { const SUPPORTS_CUSTOM_INNER_ATTRS : bool = false ; fn attrs (& self) -> & [Attribute] { & [] } fn visit_attrs (& mut self , _f : impl FnOnce (& mut AttrVec)) { } }) + } ; }
};
}
