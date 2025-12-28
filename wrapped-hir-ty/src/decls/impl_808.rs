macro_rules! deps {
    () => {
        InternedOpaqueTyId!();
        HirDatabase!();
        EarlyBinder!();
    };
}

macro_rules! impl_808 {
    () => {
        deps!();
        impl InternedOpaqueTyId { # [inline] pub fn predicates < 'db > (self , db : & 'db dyn HirDatabase) -> EarlyBinder < 'db , & 'db [Clause < 'db >] > { self . loc (db) . predicates (db) } }
    };
}

impl_808!();