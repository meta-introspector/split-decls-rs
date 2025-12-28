macro_rules! deps {
    () => {
        LateContext!();
        LateContextAndPass!();
    };
}

macro_rules! late_lint_crate_inner {
    () => {
        deps!();
        fn late_lint_crate_inner < 'tcx , T : LateLintPass < 'tcx > > (tcx : TyCtxt < 'tcx > , context : LateContext < 'tcx > , pass : T ,) { let mut cx = LateContextAndPass { context , pass } ; cx . with_lint_attrs (hir :: CRATE_HIR_ID , | cx | { lint_callback ! (cx , check_crate ,) ; tcx . hir_walk_toplevel_module (cx) ; lint_callback ! (cx , check_crate_post ,) ; }) }
    };
}

late_lint_crate_inner!()