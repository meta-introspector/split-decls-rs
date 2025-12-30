// Generated macro for check (function)
macro_rules! Depcrate_needless_question_markcheck {
() => {
// Module: crate::needless_question_mark
// Provides: {"check"}
// Dependencies: {}
fn check (cx : & LateContext < '_ > , expr : & Expr < '_ >) { if let ExprKind :: Call (path , [arg]) = expr . kind && let Res :: Def (DefKind :: Ctor (..) , ctor_id) = path . res (cx) && let Some (variant_id) = cx . tcx . opt_parent (ctor_id) && let variant = if cx . tcx . lang_items () . option_some_variant () == Some (variant_id) { "Some" } else if cx . tcx . lang_items () . result_ok_variant () == Some (variant_id) { "Ok" } else { return ; } && let ExprKind :: Match (inner_expr_with_q , _ , MatchSource :: TryDesugar (_)) = & arg . kind && let ExprKind :: Call (called , [inner_expr]) = & inner_expr_with_q . kind && let ExprKind :: Path (qpath) = called . kind && cx . tcx . qpath_is_lang_item (qpath , LangItem :: TryTraitBranch) && expr . span . eq_ctxt (inner_expr . span) && let expr_ty = cx . typeck_results () . expr_ty (expr) && let inner_ty = cx . typeck_results () . expr_ty (inner_expr) && expr_ty == inner_ty { span_lint_hir_and_then (cx , NEEDLESS_QUESTION_MARK , expr . hir_id , expr . span , format ! ("enclosing `{variant}` and `?` operator are unneeded") , | diag | { diag . multipart_suggestion (format ! ("remove the enclosing `{variant}` and `?` operator") , vec ! [(expr . span . until (inner_expr . span) , String :: new ()) , (inner_expr . span . shrink_to_hi () . to (expr . span . shrink_to_hi ()) , String :: new () ,) ,] , Applicability :: MachineApplicable ,) ; } ,) ; } }
};
}
