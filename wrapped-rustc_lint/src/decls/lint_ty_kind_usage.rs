macro_rules! deps {
    () => {
        LateContext!();
    };
}

macro_rules! lint_ty_kind_usage {
    () => {
        deps!();
        fn lint_ty_kind_usage (cx : & LateContext < '_ > , res : & Res) -> bool { if let Some (did) = res . opt_def_id () { cx . tcx . is_diagnostic_item (sym :: TyKind , did) || cx . tcx . is_diagnostic_item (sym :: IrTyKind , did) } else { false } }
    };
}

lint_ty_kind_usage!()