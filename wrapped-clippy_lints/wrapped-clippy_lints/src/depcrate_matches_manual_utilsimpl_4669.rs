// Generated macro for impl_4669 (impl)
macro_rules! Depcrate_matches_manual_utilsimpl_4669 {
() => {
// Module: crate::matches::manual_utils
// Provides: {"impl_4669"}
// Dependencies: {}
impl < 'tcx > SomeExpr < 'tcx > { pub fn new_no_negated (expr : & 'tcx Expr < 'tcx > , needs_unsafe_block : bool) -> Self { Self { expr , needs_unsafe_block , needs_negated : false , } } pub fn to_snippet_with_context (& self , cx : & LateContext < 'tcx > , ctxt : SyntaxContext , app : & mut Applicability ,) -> Sugg < 'tcx > { let sugg = Sugg :: hir_with_context (cx , self . expr , ctxt , ".." , app) ; if self . needs_negated { ! sugg } else { sugg } } }
};
}
