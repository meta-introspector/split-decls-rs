// Generated macro for check (function)
macro_rules! Depcrate_methods_from_iter_instead_of_collectcheck {
() => {
// Module: crate::methods::from_iter_instead_of_collect
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , expr : & Expr < '_ > , args : & [Expr < '_ >] , func : & Expr < '_ >) { if func . res (cx) . is_diag_item (cx , sym :: from_iter_fn) && let arg_ty = cx . typeck_results () . expr_ty (& args [0]) && let Some (iter_id) = cx . tcx . get_diagnostic_item (sym :: Iterator) && implements_trait (cx , arg_ty , iter_id , & []) { let mut app = Applicability :: MaybeIncorrect ; let turbofish = match func . kind { ExprKind :: Path (QPath :: TypeRelative (hir_ty , _)) => build_full_type (cx , hir_ty , & mut app) , ExprKind :: Path (QPath :: Resolved (Some (self_ty) , _)) => build_full_type (cx , self_ty , & mut app) , _ => return , } ; let iter_expr = sugg :: Sugg :: hir (cx , & args [0] , "..") . maybe_paren () ; let sugg = format ! ("{iter_expr}.collect::<{turbofish}>()") ; span_lint_and_sugg (cx , FROM_ITER_INSTEAD_OF_COLLECT , expr . span , "usage of `FromIterator::from_iter`" , "use `.collect()` instead of `::from_iter()`" , sugg , app ,) ; } }
};
}
