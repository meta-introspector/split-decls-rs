// Generated macro for record_rvalue_scope (function)
macro_rules! Depcrate_rvalue_scopesrecord_rvalue_scope {
() => {
// Module: crate::rvalue_scopes
// Provides: {"record_rvalue_scope"}
// Dependencies: {}
fn record_rvalue_scope (rvalue_scopes : & mut RvalueScopes , expr : & hir :: Expr < '_ > , candidate : & RvalueCandidate ,) { debug ! ("resolve_rvalue_scope(expr={expr:?}, candidate={candidate:?})") ; record_rvalue_scope_rec (rvalue_scopes , expr , candidate . lifetime , candidate . compat) }
};
}
