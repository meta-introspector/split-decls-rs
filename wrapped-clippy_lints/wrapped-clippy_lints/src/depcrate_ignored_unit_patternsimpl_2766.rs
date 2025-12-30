// Generated macro for impl_2766 (impl)
macro_rules! Depcrate_ignored_unit_patternsimpl_2766 {
() => {
// Module: crate::ignored_unit_patterns
// Provides: {"impl_2766"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for IgnoredUnitPatterns { fn check_pat (& mut self , cx : & LateContext < 'tcx > , pat : & 'tcx hir :: Pat < 'tcx >) { if matches ! (pat . kind , PatKind :: Wild) && ! pat . span . from_expansion () && cx . typeck_results () . pat_ty (pat) . peel_refs () . is_unit () { match cx . tcx . parent_hir_node (pat . hir_id) { Node :: Param (param) if matches ! (cx . tcx . parent_hir_node (param . hir_id) , Node :: Item (_)) => { return ; } , Node :: LetStmt (local) if local . ty . is_some () => { return ; } , _ => { } , } span_lint_and_sugg (cx , IGNORED_UNIT_PATTERNS , pat . span , "matching over `()` is more explicit" , "use `()` instead of `_`" , String :: from ("()") , Applicability :: MachineApplicable ,) ; } } }
};
}
