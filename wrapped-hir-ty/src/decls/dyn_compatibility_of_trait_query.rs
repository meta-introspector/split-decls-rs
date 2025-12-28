macro_rules! deps {
    () => {
        DynCompatibilityViolation!();
        HirDatabase!();
    };
}

macro_rules! dyn_compatibility_of_trait_query {
    () => {
        deps!();
        pub fn dyn_compatibility_of_trait_query (db : & dyn HirDatabase , trait_ : TraitId ,) -> Option < DynCompatibilityViolation > { let mut res = None ; _ = dyn_compatibility_of_trait_with_callback (db , trait_ , & mut | osv | { res = Some (osv) ; ControlFlow :: Break (()) }) ; res }
    };
}

dyn_compatibility_of_trait_query!();