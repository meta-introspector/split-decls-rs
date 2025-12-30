// Generated macro for trait_environment_for_body_query (function)
macro_rules! Depcrate_lowertrait_environment_for_body_query {
() => {
// Module: crate::lower
// Provides: {"trait_environment_for_body_query"}
// Dependencies: {}
pub (crate) fn trait_environment_for_body_query (db : & dyn HirDatabase , def : DefWithBodyId ,) -> Arc < TraitEnvironment < '_ > > { let Some (def) = def . as_generic_def_id (db) else { let krate = def . module (db) . krate () ; return TraitEnvironment :: empty (krate) ; } ; db . trait_environment (def) }
};
}
