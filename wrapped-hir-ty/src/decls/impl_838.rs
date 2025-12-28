macro_rules! deps {
    () => {
        HirDatabase!();
        EarlyBinder!();
        GenericPredicates!();
    };
}

macro_rules! impl_838 {
    () => {
        deps!();
        impl < 'db > GenericPredicates < 'db > { # [inline] pub fn query (db : & 'db dyn HirDatabase , def : GenericDefId) -> & 'db GenericPredicates < 'db > { & Self :: query_with_diagnostics (db , def) . 0 } # [inline] pub fn query_all (db : & 'db dyn HirDatabase , def : GenericDefId ,) -> EarlyBinder < 'db , & 'db [Clause < 'db >] > { Self :: query (db , def) . all_predicates () } # [inline] pub fn query_own (db : & 'db dyn HirDatabase , def : GenericDefId ,) -> EarlyBinder < 'db , & 'db [Clause < 'db >] > { Self :: query (db , def) . own_predicates () } # [inline] pub fn query_explicit (db : & 'db dyn HirDatabase , def : GenericDefId ,) -> EarlyBinder < 'db , & 'db [Clause < 'db >] > { Self :: query (db , def) . explicit_predicates () } # [inline] pub fn all_predicates (& self) -> EarlyBinder < 'db , & [Clause < 'db >] > { self . predicates . as_ref () . map_bound (| it | & * * it) } # [inline] pub fn own_predicates (& self) -> EarlyBinder < 'db , & [Clause < 'db >] > { self . predicates . as_ref () . map_bound (| it | & it [self . own_predicates_start as usize ..]) } # [doc = " Returns the predicates, minus the implicit `Self: Trait` predicate for a trait."] # [inline] pub fn explicit_predicates (& self) -> EarlyBinder < 'db , & [Clause < 'db >] > { self . predicates . as_ref () . map_bound (| it | { & it [usize :: from (self . parent_is_trait) .. it . len () - usize :: from (self . is_trait)] }) } }
    };
}

impl_838!()