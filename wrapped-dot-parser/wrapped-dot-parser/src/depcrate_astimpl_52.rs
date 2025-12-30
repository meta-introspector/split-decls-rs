// Generated macro for impl_52 (impl)
macro_rules! Depcrate_astimpl_52 {
() => {
// Module: crate::ast
// Provides: {"impl_52"}
// Dependencies: {}
impl < A > AttrStmt < A > { pub (crate) fn filter_map_attr < B > (self , f : & dyn Fn (A) -> Option < B >) -> AttrStmt < B > { match self { AttrStmt :: Graph (attr) => AttrStmt :: Graph (attr . filter_map_attr (f)) , AttrStmt :: Node (attr) => AttrStmt :: Node (attr . filter_map_attr (f)) , AttrStmt :: Edge (attr) => AttrStmt :: Edge (attr . filter_map_attr (f)) , } } }
};
}
