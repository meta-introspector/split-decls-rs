// Generated macro for get_parent_expr_for_hir (function)
macro_rules! Depcrateget_parent_expr_for_hir {
() => {
// Module: crate
// Provides: {"get_parent_expr_for_hir"}
// Dependencies: {}
# [doc = " This retrieves the parent for the given `HirId` if it's an expression. This is useful for"] # [doc = " constraint lints"] pub fn get_parent_expr_for_hir < 'tcx > (cx : & LateContext < 'tcx > , hir_id : HirId) -> Option < & 'tcx Expr < 'tcx > > { match cx . tcx . parent_hir_node (hir_id) { Node :: Expr (parent) => Some (parent) , _ => None , } }
};
}
