// Generated macro for impl_132 (impl)
macro_rules! Depcrate_canonicalimpl_132 {
() => {
// Module: crate::canonical
// Provides: {"impl_132"}
// Dependencies: {}
impl < A > From < AstAttrStmt < A > > for Vec < AttrStmt < A > > { fn from (val : AstAttrStmt < A >) -> Self { match val { AstAttrStmt :: Graph (list) => { let alist : AList < A > = list . into () ; alist . into_iter () . map (| attr | AttrStmt :: Graph (attr)) . collect () } AstAttrStmt :: Node (list) => { let alist : AList < A > = list . into () ; alist . into_iter () . map (| attr | AttrStmt :: Node (attr)) . collect () } AstAttrStmt :: Edge (list) => { let alist : AList < A > = list . into () ; alist . into_iter () . map (| attr | AttrStmt :: Edge (attr)) . collect () } } } }
};
}
