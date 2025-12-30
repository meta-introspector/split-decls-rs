// Generated macro for is_shadow (function)
macro_rules! Depcrate_shadowis_shadow {
() => {
// Module: crate::shadow
// Provides: {"is_shadow"}
// Dependencies: {}
fn is_shadow (cx : & LateContext < '_ > , owner : LocalDefId , first : ItemLocalId , second : ItemLocalId) -> bool { let scope_tree = cx . tcx . region_scope_tree (owner . to_def_id ()) ; if let Some (first_scope) = scope_tree . var_scope (first) && let Some (second_scope) = scope_tree . var_scope (second) { return scope_tree . is_subscope_of (second_scope , first_scope) ; } false }
};
}
