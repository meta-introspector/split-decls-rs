// Generated macro for resolve_rvalue_scopes (function)
macro_rules! Depcrate_rvalue_scopesresolve_rvalue_scopes {
() => {
// Module: crate::rvalue_scopes
// Provides: {"resolve_rvalue_scopes"}
// Dependencies: {}
pub (crate) fn resolve_rvalue_scopes < 'a , 'tcx > (fcx : & 'a FnCtxt < 'a , 'tcx > , scope_tree : & 'a ScopeTree , def_id : DefId ,) -> RvalueScopes { let tcx = & fcx . tcx ; let mut rvalue_scopes = RvalueScopes :: new () ; debug ! ("start resolving rvalue scopes, def_id={def_id:?}") ; debug ! ("rvalue_scope: rvalue_candidates={:?}" , scope_tree . rvalue_candidates) ; for (& hir_id , candidate) in & scope_tree . rvalue_candidates { let Node :: Expr (expr) = tcx . hir_node (hir_id) else { bug ! ("hir node does not exist") } ; record_rvalue_scope (& mut rvalue_scopes , expr , candidate) ; } rvalue_scopes }
};
}
