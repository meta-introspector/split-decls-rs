// Generated macro for resolver_for_scope_ (function)
macro_rules! Depcrate_resolverresolver_for_scope_ {
() => {
// Module: crate::resolver
// Provides: {"resolver_for_scope_"}
// Dependencies: {}
fn resolver_for_scope_ < 'db > (db : & 'db dyn DefDatabase , scopes : Arc < ExprScopes > , scope_id : Option < ScopeId > , mut r : Resolver < 'db > , owner : DefWithBodyId ,) -> Resolver < 'db > { let scope_chain = scopes . scope_chain (scope_id) . collect :: < Vec < _ > > () ; r . scopes . reserve (scope_chain . len ()) ; for scope in scope_chain . into_iter () . rev () { if let Some (block) = scopes . block (scope) { let def_map = block_def_map (db , block) ; let local_def_map = block . lookup (db) . module . only_local_def_map (db) ; r = r . push_block_scope (def_map , local_def_map , DefMap :: ROOT) ; } if let Some (macro_id) = scopes . macro_def (scope) { r = r . push_scope (Scope :: MacroDefScope (* * macro_id)) ; } r = r . push_expr_scope (owner , Arc :: clone (& scopes) , scope) ; } r }
};
}
