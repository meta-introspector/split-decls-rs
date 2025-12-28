macro_rules! deps {
    () => {
        EarlyBinder!();
        HirDatabase!();
        InternedOpaqueTyId!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl InternedOpaqueTyId { # [inline] pub fn predicates < 'db > (self , db : & 'db dyn HirDatabase) -> EarlyBinder < 'db , & 'db [Clause < 'db >] > { self . loc (db) . predicates (db) } }
    };
}

impl_126!();