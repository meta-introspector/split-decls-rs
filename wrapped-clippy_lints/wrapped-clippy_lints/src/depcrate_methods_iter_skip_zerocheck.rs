// Generated macro for check (function)
macro_rules! Depcrate_methods_iter_skip_zerocheck {
() => {
// Module: crate::methods::iter_skip_zero
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , arg_expr : & Expr < '_ >) { if ! expr . span . from_expansion () && cx . ty_based_def (expr) . opt_parent (cx) . is_diag_item (cx , sym :: Iterator) && let Some (arg) = ConstEvalCtxt :: new (cx) . eval_local (arg_expr , expr . span . ctxt ()) . and_then (| constant | { if let Constant :: Int (arg) = constant { Some (arg) } else { None } }) && arg == 0 && ! is_from_proc_macro (cx , expr) { span_lint_and_then (cx , ITER_SKIP_ZERO , arg_expr . span , "usage of `.skip(0)`" , | diag | { diag . span_suggestion (arg_expr . span , "if you meant to skip the first element, use" , "1" , Applicability :: MaybeIncorrect ,) . note ("this call to `skip` does nothing and is useless; remove it") ; }) ; } }
};
}
