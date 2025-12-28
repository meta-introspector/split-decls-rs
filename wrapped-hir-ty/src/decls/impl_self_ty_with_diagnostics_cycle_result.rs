macro_rules! deps {
    () => {
        EarlyBinder!();
        HirDatabase!();
        Diagnostics!();
    };
}

macro_rules! impl_self_ty_with_diagnostics_cycle_result {
    () => {
        deps!();
        pub (crate) fn impl_self_ty_with_diagnostics_cycle_result (db : & dyn HirDatabase , _impl_id : ImplId ,) -> (EarlyBinder < '_ , Ty < '_ > > , Diagnostics) { (EarlyBinder :: bind (Ty :: new_error (DbInterner :: new_with (db , None , None) , ErrorGuaranteed)) , None) }
    };
}

impl_self_ty_with_diagnostics_cycle_result!()