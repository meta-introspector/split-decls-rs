// Generated macro for check_stmt (function)
macro_rules! Depcrate_returns_needless_return_with_question_markcheck_stmt {
() => {
// Module: crate::returns::needless_return_with_question_mark
// Provides: {"check_stmt"}
// Dependencies: {}
pub (super) fn check_stmt < 'tcx > (cx : & LateContext < 'tcx > , stmt : & 'tcx Stmt < '_ >) { if ! stmt . span . in_external_macro (cx . sess () . source_map ()) && let StmtKind :: Semi (expr) = stmt . kind && let ExprKind :: Ret (Some (ret)) = expr . kind && let ExprKind :: Match (maybe_cons , _ , MatchSource :: TryDesugar (_)) = ret . kind && let ExprKind :: Call (_ , [maybe_result_err]) = maybe_cons . kind && let ExprKind :: Call (maybe_constr , _) = maybe_result_err . kind && maybe_constr . res (cx) . ctor_parent (cx) . is_lang_item (cx , ResultErr) && let OwnerNode :: Item (item) = cx . tcx . hir_owner_node (cx . tcx . hir_get_parent_item (expr . hir_id)) && let ItemKind :: Fn { body , .. } = item . kind && let block = cx . tcx . hir_body (body) . value && let ExprKind :: Block (block , _) = block . kind && ! is_inside_let_else (cx . tcx , expr) && let [.. , final_stmt] = block . stmts && final_stmt . hir_id != stmt . hir_id && ! is_from_proc_macro (cx , expr) && ! stmt_needs_never_type (cx , stmt . hir_id) { span_lint_and_sugg (cx , NEEDLESS_RETURN_WITH_QUESTION_MARK , expr . span . until (ret . span) , "unneeded `return` statement with `?` operator" , "remove it" , String :: new () , Applicability :: MachineApplicable ,) ; } }
};
}
