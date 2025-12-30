// Generated macro for safe_to_move_scrutinee (function)
macro_rules! Depcrate_matches_manual_unwrap_orsafe_to_move_scrutinee {
() => {
// Module: crate::matches::manual_unwrap_or
// Provides: {"safe_to_move_scrutinee"}
// Dependencies: {}
# [doc = " Checks whether it is safe to move scrutinee."] # [doc = " It is not safe to move if:"] # [doc = "     1. `scrutinee` is a `Result` that doesn't implemenet `Copy`, mainly because the `Err`"] # [doc = "        variant is not copyable."] # [doc = "     2. `expr` is a local variable that is used after the if-let-else expression."] # [doc = " ```rust,ignore"] # [doc = " let foo: Result<usize, String> = Ok(0);"] # [doc = " let v = if let Ok(v) = foo { v } else { 1 };"] # [doc = " let bar = foo;"] # [doc = " ```"] fn safe_to_move_scrutinee (cx : & LateContext < '_ > , expr : & Expr < '_ > , scrutinee : & Expr < '_ >) -> bool { if let Some (hir_id) = scrutinee . res_local_id () && let scrutinee_ty = cx . typeck_results () . expr_ty (scrutinee) && scrutinee_ty . is_diag_item (cx , sym :: Result) && ! is_copy (cx , scrutinee_ty) && local_used_after_expr (cx , hir_id , expr) { false } else { true } }
};
}
