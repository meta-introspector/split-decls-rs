// Generated macro for impl_131 (impl)
macro_rules! Depcrate_canonicalimpl_131 {
() => {
// Module: crate::canonical
// Provides: {"impl_131"}
// Dependencies: {}
impl < A > AttrStmt < A > { fn filter_map < F , B > (self , f : F) -> Option < AttrStmt < B > > where F : Fn (A) -> Option < B > , { match self { AttrStmt :: Graph (a) => f (a) . map (AttrStmt :: Graph) , AttrStmt :: Node (a) => f (a) . map (AttrStmt :: Node) , AttrStmt :: Edge (a) => f (a) . map (AttrStmt :: Edge) , } } }
};
}
