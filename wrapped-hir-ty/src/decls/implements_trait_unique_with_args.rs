macro_rules! deps {
    () => {
        TraitEnvironment!();
        HirDatabase!();
    };
}

macro_rules! implements_trait_unique_with_args {
    () => {
        deps!();
        # [doc = " This should not be used in `hir-ty`, only in `hir`."] pub fn implements_trait_unique_with_args < 'db > (db : & 'db dyn HirDatabase , env : Arc < TraitEnvironment < 'db > > , trait_ : TraitId , args : GenericArgs < 'db > ,) -> bool { implements_trait_unique_impl (db , env , trait_ , & mut | _ | args) }
    };
}

implements_trait_unique_with_args!();