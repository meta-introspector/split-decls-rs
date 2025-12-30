// Generated macro for get_enclosing_loop_or_multi_call_closure (function)
macro_rules! Depcrateget_enclosing_loop_or_multi_call_closure {
() => {
// Module: crate
// Provides: {"get_enclosing_loop_or_multi_call_closure"}
// Dependencies: {}
# [doc = " Gets the loop or closure enclosing the given expression, if any."] pub fn get_enclosing_loop_or_multi_call_closure < 'tcx > (cx : & LateContext < 'tcx > , expr : & Expr < '_ > ,) -> Option < & 'tcx Expr < 'tcx > > { for (_ , node) in cx . tcx . hir_parent_iter (expr . hir_id) { match node { Node :: Expr (e) => match e . kind { ExprKind :: Closure { .. } if let rustc_ty :: Closure (_ , subs) = cx . typeck_results () . expr_ty (e) . kind () && subs . as_closure () . kind () == ClosureKind :: FnOnce => { } , ExprKind :: Closure { .. } | ExprKind :: Loop (..) => return Some (e) , _ => () , } , Node :: Stmt (_) | Node :: Block (_) | Node :: LetStmt (_) | Node :: Arm (_) | Node :: ExprField (_) => () , _ => break , } } None }
};
}
