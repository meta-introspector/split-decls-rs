macro_rules! var_name {
    () => {
        fn var_name (tcx : TyCtxt < '_ > , var_hir_id : HirId) -> Symbol { tcx . hir_name (var_hir_id) }
    };
}

var_name!();