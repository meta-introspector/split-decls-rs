// Generated macro for check (function)
macro_rules! Depcrate_methods_filter_map_identitycheck {
() => {
// Module: crate::methods::filter_map_identity
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ > , filter_map_arg : & hir :: Expr < '_ > , filter_map_span : Span) { if cx . ty_based_def (expr) . opt_parent (cx) . is_diag_item (cx , sym :: Iterator) && let Some (applicability) = is_identity (cx , filter_map_arg) { if let ExprKind :: MethodCall (_ , recv , ..) = expr . kind && let ExprKind :: MethodCall (_ , recv2 , ..) = recv . kind && let ExprKind :: Array (arr) = recv2 . kind && arr . is_empty () { return ; } span_lint_and_sugg (cx , FILTER_MAP_IDENTITY , filter_map_span . with_hi (expr . span . hi ()) , "use of `filter_map` with an identity function" , "try" , "flatten()" . to_string () , applicability ,) ; } }
};
}
