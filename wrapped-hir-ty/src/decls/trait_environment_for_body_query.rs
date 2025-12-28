macro_rules! deps {
    () => {
        HirDatabase!();
        TraitEnvironment!();
    };
}

macro_rules! trait_environment_for_body_query {
    () => {
        deps!();
        pub (crate) fn trait_environment_for_body_query (db : & dyn HirDatabase , def : DefWithBodyId ,) -> Arc < TraitEnvironment < '_ > > { let Some (def) = def . as_generic_def_id (db) else { let krate = def . module (db) . krate () ; return TraitEnvironment :: empty (krate) ; } ; db . trait_environment (def) }
    };
}

trait_environment_for_body_query!()