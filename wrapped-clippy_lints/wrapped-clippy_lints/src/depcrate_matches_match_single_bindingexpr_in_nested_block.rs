// Generated macro for expr_in_nested_block (function)
macro_rules! Depcrate_matches_match_single_bindingexpr_in_nested_block {
() => {
// Module: crate::matches::match_single_binding
// Provides: {"expr_in_nested_block"}
// Dependencies: {}
fn expr_in_nested_block (cx : & LateContext < '_ > , match_expr : & Expr < '_ >) -> bool { if let Node :: Block (block) = cx . tcx . parent_hir_node (match_expr . hir_id) { return block . expr . map_or_else (| | matches ! (block . stmts , [_]) , | _ | block . stmts . is_empty ()) ; } false }
};
}
