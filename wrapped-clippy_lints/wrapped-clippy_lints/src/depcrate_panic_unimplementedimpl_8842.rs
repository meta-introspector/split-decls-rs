// Generated macro for impl_8842 (impl)
macro_rules! Depcrate_panic_unimplementedimpl_8842 {
() => {
// Module: crate::panic_unimplemented
// Provides: {"impl_8842"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for PanicUnimplemented { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if let Some (macro_call) = root_macro_call_first_node (cx , expr) { if is_panic (cx , macro_call . def_id) { if is_inside_always_const_context (cx . tcx , expr . hir_id) || self . allow_panic_in_tests && is_in_test (cx . tcx , expr . hir_id) { return ; } span_lint (cx , PANIC , macro_call . span , "`panic` should not be present in production code" ,) ; return ; } match cx . tcx . get_diagnostic_name (macro_call . def_id) { Some (sym :: todo_macro) => { span_lint (cx , TODO , macro_call . span , "`todo` should not be present in production code" ,) ; } , Some (sym :: unimplemented_macro) => { span_lint (cx , UNIMPLEMENTED , macro_call . span , "`unimplemented` should not be present in production code" ,) ; } , Some (sym :: unreachable_macro) => { span_lint (cx , UNREACHABLE , macro_call . span , "usage of the `unreachable!` macro") ; } , _ => { } , } } else if let ExprKind :: Call (func , [_]) = expr . kind && let ExprKind :: Path (QPath :: Resolved (None , expr_path)) = func . kind && let Res :: Def (DefKind :: Fn , def_id) = expr_path . res && cx . tcx . is_diagnostic_item (sym :: panic_any , def_id) { if is_inside_always_const_context (cx . tcx , expr . hir_id) || self . allow_panic_in_tests && is_in_test (cx . tcx , expr . hir_id) { return ; } span_lint (cx , PANIC , expr . span , "`panic_any` should not be present in production code" ,) ; } } }
};
}
