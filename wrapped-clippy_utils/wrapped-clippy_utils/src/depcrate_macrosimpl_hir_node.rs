// Generated macro for impl_hir_node (macro)
macro_rules! Depcrate_macrosimpl_hir_node {
() => {
// Module: crate::macros
// Provides: {"impl_hir_node"}
// Dependencies: {}
macro_rules ! impl_hir_node { ($ ($ t : ident) ,*) => { $ (impl HirNode for hir ::$ t <'_ > { fn hir_id (& self) -> HirId { self . hir_id } fn span (& self) -> Span { self . span } }) * } ; }
};
}
