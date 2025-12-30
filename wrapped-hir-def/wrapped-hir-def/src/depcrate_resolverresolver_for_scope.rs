// Generated macro for resolver_for_scope (function)
macro_rules! Depcrate_resolverresolver_for_scope {
() => {
// Module: crate::resolver
// Provides: {"resolver_for_scope"}
// Dependencies: {}
pub fn resolver_for_scope (db : & dyn DefDatabase , owner : DefWithBodyId , scope_id : Option < ScopeId > ,) -> Resolver < '_ > { let r = owner . resolver (db) ; let scopes = db . expr_scopes (owner) ; resolver_for_scope_ (db , scopes , scope_id , r , owner) }
};
}
