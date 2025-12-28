macro_rules! analysis {
    () => {
        # [doc = " Runs the type-checking, region checking and other miscellaneous analysis"] # [doc = " passes on the crate."] fn analysis (tcx : TyCtxt < '_ > , () : ()) { run_required_analyses (tcx) ; let sess = tcx . sess ; if let Some (guar) = sess . dcx () . has_errors_excluding_lint_errors () { guar . raise_fatal () ; } sess . time ("misc_checking_3" , | | { parallel ! ({ tcx . ensure_ok () . effective_visibilities (()) ; parallel ! ({ tcx . par_hir_for_each_module (| module | { tcx . ensure_ok () . check_private_in_public (module) }) } , { tcx . par_hir_for_each_module (| module | { tcx . ensure_ok () . check_mod_deathness (module) }) ; } , { sess . time ("lint_checking" , || { rustc_lint :: check_crate (tcx) ; }) ; } , { tcx . ensure_ok () . clashing_extern_declarations (()) ; }) ; } , { sess . time ("privacy_checking_modules" , || { tcx . par_hir_for_each_module (| module | { tcx . ensure_ok () . check_mod_privacy (module) ; }) ; }) ; }) ; sess . time ("check_lint_expectations" , | | tcx . ensure_ok () . check_expectations (None)) ; let _ = tcx . all_diagnostic_items (()) ; }) ; }
    };
}

analysis!()