// Generated macro for check (function)
macro_rules! Depcrate_methods_inspect_for_eachcheck {
() => {
// Module: crate::methods::inspect_for_each
// Provides: {"check"}
// Dependencies: {}
# [doc = " lint use of `inspect().for_each()` for `Iterators`"] pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < '_ > , inspect_span : Span) { if cx . ty_based_def (expr) . opt_parent (cx) . is_diag_item (cx , sym :: Iterator) { let msg = "called `inspect(..).for_each(..)` on an `Iterator`" ; let hint = "move the code from `inspect(..)` to `for_each(..)` and remove the `inspect(..)`" ; span_lint_and_help (cx , INSPECT_FOR_EACH , inspect_span . with_hi (expr . span . hi ()) , msg , None , hint ,) ; } }
};
}
