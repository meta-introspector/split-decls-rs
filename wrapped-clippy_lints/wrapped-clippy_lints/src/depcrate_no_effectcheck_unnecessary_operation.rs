// Generated macro for check_unnecessary_operation (function)
macro_rules! Depcrate_no_effectcheck_unnecessary_operation {
() => {
// Module: crate::no_effect
// Provides: {"check_unnecessary_operation"}
// Dependencies: {}
fn check_unnecessary_operation (cx : & LateContext < '_ > , stmt : & Stmt < '_ >) { if let StmtKind :: Semi (expr) = stmt . kind && ! stmt . span . in_external_macro (cx . sess () . source_map ()) && let ctxt = stmt . span . ctxt () && expr . range_span () . unwrap_or (expr . span) . ctxt () == ctxt && let Some (reduced) = reduce_expression (cx , expr) && reduced . iter () . all (| e | e . span . ctxt () == ctxt) { if let ExprKind :: Index (..) = & expr . kind { if ! is_inside_always_const_context (cx . tcx , expr . hir_id) && let [arr , func] = & * reduced && let Some (arr) = arr . span . get_source_text (cx) && let Some (func) = func . span . get_source_text (cx) { span_lint_hir_and_then (cx , UNNECESSARY_OPERATION , expr . hir_id , stmt . span , "unnecessary operation" , | diag | { diag . span_suggestion (stmt . span , "statement can be written as" , format ! ("assert!({arr}.len() > {func});") , Applicability :: MaybeIncorrect ,) ; } ,) ; } } else { let mut snippet = String :: new () ; for e in reduced { if let Some (snip) = e . span . get_source_text (cx) { snippet . push_str (& snip) ; snippet . push_str ("; ") ; } else { return ; } } snippet . pop () ; span_lint_hir_and_then (cx , UNNECESSARY_OPERATION , expr . hir_id , stmt . span , "unnecessary operation" , | diag | { diag . span_suggestion (stmt . span , "statement can be reduced to" , snippet , Applicability :: MachineApplicable ,) ; } ,) ; } } }
};
}
