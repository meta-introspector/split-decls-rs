macro_rules! check_crate {
    () => {
        # [doc = " Performs lint checking on a crate."] pub fn check_crate < 'tcx > (tcx : TyCtxt < 'tcx >) { join (| | { tcx . sess . time ("crate_lints" , | | { late_lint_crate (tcx) ; }) ; } , | | { tcx . sess . time ("module_lints" , | | { tcx . par_hir_for_each_module (| module | tcx . ensure_ok () . lint_mod (module)) ; }) ; } ,) ; }
    };
}

check_crate!();