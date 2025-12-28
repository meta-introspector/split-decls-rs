macro_rules! deps {
    () => {
        GenericPredicates!();
        TraitEnvironment!();
        HirDatabase!();
    };
}

macro_rules! trait_environment_query {
    () => {
        deps!();
        pub (crate) fn trait_environment_query < 'db > (db : & 'db dyn HirDatabase , def : GenericDefId ,) -> Arc < TraitEnvironment < 'db > > { let module = def . module (db) ; let interner = DbInterner :: new_with (db , Some (module . krate ()) , module . containing_block ()) ; let predicates = GenericPredicates :: query_all (db , def) ; let traits_in_scope = predicates . iter_identity_copied () . filter_map (| pred | match pred . kind () . skip_binder () { ClauseKind :: Trait (tr) => Some ((tr . self_ty () , tr . def_id () . 0)) , _ => None , }) . collect () ; let clauses = rustc_type_ir :: elaborate :: elaborate (interner , predicates . iter_identity_copied ()) ; let clauses = Clauses :: new_from_iter (interner , clauses) ; let env = ParamEnv { clauses } ; TraitEnvironment :: new (module . krate () , module . containing_block () , traits_in_scope , env) }
    };
}

trait_environment_query!()