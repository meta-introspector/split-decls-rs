macro_rules! deps {
    () => {
        LateContext!();
        LateContextAndPass!();
    };
}

macro_rules! late_lint_mod_inner {
    () => {
        deps!();
        fn late_lint_mod_inner < 'tcx , T : LateLintPass < 'tcx > > (tcx : TyCtxt < 'tcx > , module_def_id : LocalModDefId , context : LateContext < 'tcx > , pass : T ,) { let mut cx = LateContextAndPass { context , pass } ; let (module , _span , hir_id) = tcx . hir_get_module (module_def_id) ; cx . with_lint_attrs (hir_id , | cx | { if hir_id == hir :: CRATE_HIR_ID { lint_callback ! (cx , check_crate ,) ; } cx . process_mod (module , hir_id) ; if hir_id == hir :: CRATE_HIR_ID { lint_callback ! (cx , check_crate_post ,) ; } }) ; }
    };
}

late_lint_mod_inner!();