// Generated macro for is_child_of_cast (function)
macro_rules! Depcrate_casts_cast_slice_different_sizesis_child_of_cast {
() => {
// Module: crate::casts::cast_slice_different_sizes
// Provides: {"is_child_of_cast"}
// Dependencies: {}
fn is_child_of_cast (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { let parent = cx . tcx . parent_hir_node (expr . hir_id) ; let expr = match parent { Node :: Block (block) => { if let Some (parent_expr) = block . expr { parent_expr } else { return false ; } } , Node :: Expr (expr) => expr , _ => return false , } ; matches ! (expr . kind , ExprKind :: Cast (..)) }
};
}
