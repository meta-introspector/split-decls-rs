// Generated macro for is_inferred_ret_closure (function)
macro_rules! Depcrate_question_markis_inferred_ret_closure {
() => {
// Module: crate::question_mark
// Provides: {"is_inferred_ret_closure"}
// Dependencies: {}
fn is_inferred_ret_closure (expr : & Expr < '_ >) -> bool { let ExprKind :: Closure (closure) = expr . kind else { return false ; } ; match closure . fn_decl . output { FnRetTy :: Return (ret_ty) => ret_ty . is_suggestable_infer_ty () , FnRetTy :: DefaultReturn (_) => true , } }
};
}
