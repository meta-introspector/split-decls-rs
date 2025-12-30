// Generated macro for check (function)
macro_rules! Depcrate_loops_infinite_loopcheck {
() => {
// Module: crate::loops::infinite_loop
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & Expr < 'tcx > , loop_block : & 'tcx hir :: Block < '_ > , label : Option < Label > ,) { if is_lint_allowed (cx , INFINITE_LOOP , expr . hir_id) { return ; } let Some (parent_fn_ret) = get_parent_fn_ret_ty (cx , expr) else { return ; } ; if is_never_return (parent_fn_ret) { return ; } if is_inside_unawaited_async_block (cx , expr) { return ; } if expr . span . in_external_macro (cx . sess () . source_map ()) || is_from_proc_macro (cx , expr) { return ; } let mut loop_visitor = LoopVisitor { cx , label , inner_labels : label . into_iter () . collect () , loop_depth : 0 , is_finite : false , } ; loop_visitor . visit_block (loop_block) ; let is_finite_loop = loop_visitor . is_finite ; if ! is_finite_loop { span_lint_and_then (cx , INFINITE_LOOP , expr . span , "infinite loop detected" , | diag | { if let FnRetTy :: DefaultReturn (ret_span) = parent_fn_ret { diag . span_suggestion (ret_span , "if this is intentional, consider specifying `!` as function return" , " -> !" , Applicability :: MaybeIncorrect ,) ; } else { diag . help ("if this is not intended, try adding a `break` or `return` condition in the loop") ; } }) ; } }
};
}
