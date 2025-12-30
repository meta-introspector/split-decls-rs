// Generated macro for lint_expr (function)
macro_rules! Depcrate_danglinglint_expr {
() => {
// Module: crate::dangling
// Provides: {"lint_expr"}
// Dependencies: {}
fn lint_expr (cx : & LateContext < '_ > , expr : & Expr < '_ >) { if let ExprKind :: MethodCall (method , receiver , _args , _span) = expr . kind && is_temporary_rvalue (receiver) && let ty = cx . typeck_results () . expr_ty (receiver) && owns_allocation (cx . tcx , ty) && let Some (fn_id) = cx . typeck_results () . type_dependent_def_id (expr . hir_id) && find_attr ! (cx . tcx . get_all_attrs (fn_id) , AttributeKind :: AsPtr (_)) { cx . tcx . emit_node_span_lint (DANGLING_POINTERS_FROM_TEMPORARIES , expr . hir_id , method . ident . span , DanglingPointersFromTemporaries { callee : method . ident , ty , ptr_span : method . ident . span , temporary_span : receiver . span , } ,) } }
};
}
