// Generated macro for impl_38 (impl)
macro_rules! Depcrate_navigation_targetimpl_38 {
() => {
// Module: crate::navigation_target
// Provides: {"impl_38"}
// Dependencies: {}
impl ToNavFromAst for hir :: Function { const KIND : SymbolKind = SymbolKind :: Function ; fn container_name (self , db : & RootDatabase) -> Option < SmolStr > { container_name (db , self , self . krate (db) . edition (db)) } }
};
}
