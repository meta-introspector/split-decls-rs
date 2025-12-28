macro_rules! deps {
    () => {
        Diagnostics!();
        HirDatabase!();
    };
}

macro_rules! const_param_ty_with_diagnostics_cycle_result {
    () => {
        deps!();
        pub (crate) fn const_param_ty_with_diagnostics_cycle_result < 'db > (db : & 'db dyn HirDatabase , _ : crate :: db :: HirDatabaseData , def : ConstParamId ,) -> (Ty < 'db > , Diagnostics) { let resolver = def . parent () . resolver (db) ; let interner = DbInterner :: new_with (db , Some (resolver . krate ()) , None) ; (Ty :: new_error (interner , ErrorGuaranteed) , None) }
    };
}

const_param_ty_with_diagnostics_cycle_result!();