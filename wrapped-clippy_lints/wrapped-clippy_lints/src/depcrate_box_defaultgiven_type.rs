// Generated macro for given_type (function)
macro_rules! Depcrate_box_defaultgiven_type {
() => {
// Module: crate::box_default
// Provides: {"given_type"}
// Dependencies: {}
fn given_type (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { match cx . tcx . parent_hir_node (expr . hir_id) { Node :: LetStmt (LetStmt { ty : Some (ty) , .. }) => { let mut v = InferVisitor :: default () ; v . visit_ty_unambig (ty) ; ! v . 0 } , Node :: Expr (Expr { kind : ExprKind :: Call (path , args) , .. }) | Node :: Block (Block { expr : Some (Expr { kind : ExprKind :: Call (path , args) , .. }) , .. }) => { if let Some (index) = args . iter () . position (| arg | arg . hir_id == expr . hir_id) && let Some (sig) = expr_sig (cx , path) && let Some (input) = sig . input (index) && let Some (input_ty) = input . no_bound_vars () { input_ty == cx . typeck_results () . expr_ty_adjusted (expr) } else { false } } , _ => false , } }
};
}
