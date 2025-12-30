// Generated macro for trait_environment_query (function)
macro_rules! Depcrate_lowertrait_environment_query {
() => {
// Module: crate::lower
// Provides: {"trait_environment_query"}
// Dependencies: {}
pub (crate) fn trait_environment_query < 'db > (db : & 'db dyn HirDatabase , def : GenericDefId ,) -> Arc < TraitEnvironment < 'db > > { let module = def . module (db) ; let interner = DbInterner :: new_with (db , Some (module . krate ()) , module . containing_block ()) ; let predicates = GenericPredicates :: query_all (db , def) ; let traits_in_scope = predicates . iter_identity_copied () . filter_map (| pred | match pred . kind () . skip_binder () { ClauseKind :: Trait (tr) => Some ((tr . self_ty () , tr . def_id () . 0)) , _ => None , }) . collect () ; let clauses = rustc_type_ir :: elaborate :: elaborate (interner , predicates . iter_identity_copied ()) ; let clauses = Clauses :: new_from_iter (interner , clauses) ; let env = ParamEnv { clauses } ; TraitEnvironment :: new (module . krate () , module . containing_block () , traits_in_scope , env) }
};
}
