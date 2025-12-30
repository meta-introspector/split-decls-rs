// Generated macro for check (function)
macro_rules! Depcrate_methods_flat_map_identitycheck {
() => {
// Module: crate::methods::flat_map_identity
// Provides: {"check"}
// Dependencies: {}
# [doc = " lint use of `flat_map` for `Iterators` where `flatten` would be sufficient"] pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < '_ > , flat_map_arg : & 'tcx hir :: Expr < '_ > , flat_map_span : Span ,) { if cx . ty_based_def (expr) . opt_parent (cx) . is_diag_item (cx , sym :: Iterator) && is_expr_untyped_identity_function (cx , flat_map_arg) { span_lint_and_sugg (cx , FLAT_MAP_IDENTITY , flat_map_span . with_hi (expr . span . hi ()) , "use of `flat_map` with an identity function" , "try" , "flatten()" . to_string () , Applicability :: MachineApplicable ,) ; } }
};
}
