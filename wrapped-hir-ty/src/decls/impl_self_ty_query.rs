macro_rules! deps {
    () => {
        HirDatabase!();
        EarlyBinder!();
    };
}

macro_rules! impl_self_ty_query {
    () => {
        deps!();
        pub (crate) fn impl_self_ty_query < 'db > (db : & 'db dyn HirDatabase , impl_id : ImplId ,) -> EarlyBinder < 'db , Ty < 'db > > { db . impl_self_ty_with_diagnostics (impl_id) . 0 }
    };
}

impl_self_ty_query!();