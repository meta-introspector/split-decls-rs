// Generated macro for impl_39 (impl)
macro_rules! Depcrate_navigation_targetimpl_39 {
() => {
// Module: crate::navigation_target
// Provides: {"impl_39"}
// Dependencies: {}
impl ToNavFromAst for hir :: Const { const KIND : SymbolKind = SymbolKind :: Const ; fn container_name (self , db : & RootDatabase) -> Option < SmolStr > { container_name (db , self , self . krate (db) . edition (db)) } }
};
}
