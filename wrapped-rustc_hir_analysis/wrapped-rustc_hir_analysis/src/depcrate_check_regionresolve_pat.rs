// Generated macro for resolve_pat (function)
macro_rules! Depcrate_check_regionresolve_pat {
() => {
// Module: crate::check::region
// Provides: {"resolve_pat"}
// Dependencies: {}
# [tracing :: instrument (level = "debug" , skip (visitor))] fn resolve_pat < 'tcx > (visitor : & mut ScopeResolutionVisitor < 'tcx > , pat : & 'tcx hir :: Pat < 'tcx >) { if let PatKind :: Binding (..) = pat . kind { record_var_lifetime (visitor , pat . hir_id . local_id) ; } intravisit :: walk_pat (visitor , pat) ; }
};
}
