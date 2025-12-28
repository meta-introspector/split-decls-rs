macro_rules! deps {
    () => {
        TraitEnvironment!();
        HirDatabase!();
    };
}

macro_rules! implements_trait_unique {
    () => {
        deps!();
        # [doc = " This should not be used in `hir-ty`, only in `hir`."] pub fn implements_trait_unique < 'db > (ty : Ty < 'db > , db : & 'db dyn HirDatabase , env : Arc < TraitEnvironment < 'db > > , trait_ : TraitId ,) -> bool { implements_trait_unique_impl (db , env , trait_ , & mut | infcx | { infcx . fill_rest_fresh_args (trait_ . into () , [ty . into ()]) }) }
    };
}

implements_trait_unique!();