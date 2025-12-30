// Generated macro for check (function)
macro_rules! Depcrate_methods_filter_map_nextcheck {
() => {
// Module: crate::methods::filter_map_next
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < '_ > , recv : & 'tcx hir :: Expr < '_ > , arg : & 'tcx hir :: Expr < '_ > , msrv : Msrv ,) { if cx . ty_based_def (expr) . opt_parent (cx) . is_diag_item (cx , sym :: Iterator) { if ! msrv . meets (cx , msrvs :: ITERATOR_FIND_MAP) { return ; } let msg = "called `filter_map(..).next()` on an `Iterator`. This is more succinctly expressed by calling \
                   `.find_map(..)` instead" ; let filter_snippet = snippet (cx , arg . span , "..") ; if filter_snippet . lines () . count () <= 1 { let iter_snippet = snippet (cx , recv . span , "..") ; span_lint_and_sugg (cx , FILTER_MAP_NEXT , expr . span , msg , "try" , format ! ("{iter_snippet}.find_map({filter_snippet})") , Applicability :: MachineApplicable ,) ; } else { span_lint (cx , FILTER_MAP_NEXT , expr . span , msg) ; } } }
};
}
