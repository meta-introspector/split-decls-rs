// Generated macro for impl_145 (impl)
macro_rules! Depcrate_invocation_collector_nodeimpl_145 {
() => {
// Module: crate::invocation_collector_node
// Provides: {"impl_145"}
// Dependencies: {}
impl HasAttrs for ExpandedCrate { const SUPPORTS_CUSTOM_INNER_ATTRS : bool = false ; fn attrs (& self) -> & [Attribute] { self . 0 . attrs . as_slice () } fn visit_attrs (& mut self , f : impl FnOnce (& mut AttrVec)) { f (& mut self . 0 . attrs) ; } }
};
}
