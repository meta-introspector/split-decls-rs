// Generated macro for check (function)
macro_rules! Depcrate_methods_skip_while_nextcheck {
() => {
// Module: crate::methods::skip_while_next
// Provides: {"check"}
// Dependencies: {}
# [doc = " lint use of `skip_while().next()` for `Iterators`"] pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < '_ >) { if cx . ty_based_def (expr) . opt_parent (cx) . is_diag_item (cx , sym :: Iterator) { span_lint_and_help (cx , SKIP_WHILE_NEXT , expr . span , "called `skip_while(<p>).next()` on an `Iterator`" , None , "this is more succinctly expressed by calling `.find(!<p>)` instead" ,) ; } }
};
}
