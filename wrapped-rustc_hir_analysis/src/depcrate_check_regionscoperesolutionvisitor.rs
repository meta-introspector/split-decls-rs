// Generated macro for ScopeResolutionVisitor (struct)
macro_rules! Depcrate_check_regionScopeResolutionVisitor {
() => {
// Module: crate::check::region
// Provides: {"ScopeResolutionVisitor"}
// Dependencies: {}
struct ScopeResolutionVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , scope_tree : ScopeTree , cx : Context , extended_super_lets : FxHashMap < hir :: ItemLocalId , ExtendedTemporaryScope > , }
};
}
