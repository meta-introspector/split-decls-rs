// Generated macro for check (function)
macro_rules! Depcrate_methods_iter_skip_nextcheck {
() => {
// Module: crate::methods::iter_skip_next
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ > , recv : & hir :: Expr < '_ > , arg : & hir :: Expr < '_ >) { if cx . ty_based_def (expr) . opt_parent (cx) . is_diag_item (cx , sym :: Iterator) { let mut applicability = Applicability :: MachineApplicable ; span_lint_and_then (cx , ITER_SKIP_NEXT , expr . span . trim_start (recv . span) . unwrap () , "called `skip(..).next()` on an iterator" , | diag | { if let Some (id) = recv . res_local_id () && let Node :: Pat (pat) = cx . tcx . hir_node (id) && let PatKind :: Binding (ann , _ , _ , _) = pat . kind && ann != BindingMode :: MUT { applicability = Applicability :: Unspecified ; diag . span_help (pat . span , format ! ("for this change `{}` has to be mutable" , snippet (cx , pat . span , "..")) ,) ; } diag . span_suggestion (expr . span . trim_start (recv . span) . unwrap () , "use `nth` instead" , format ! (".nth({})" , snippet (cx , arg . span , "..")) , applicability ,) ; } ,) ; } }
};
}
