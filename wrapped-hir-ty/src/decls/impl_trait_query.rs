macro_rules! deps {
    () => {
        HirDatabase!();
        EarlyBinder!();
    };
}

macro_rules! impl_trait_query {
    () => {
        deps!();
        pub (crate) fn impl_trait_query < 'db > (db : & 'db dyn HirDatabase , impl_id : ImplId ,) -> Option < EarlyBinder < 'db , TraitRef < 'db > > > { db . impl_trait_with_diagnostics (impl_id) . map (| it | it . 0) }
    };
}

impl_trait_query!()