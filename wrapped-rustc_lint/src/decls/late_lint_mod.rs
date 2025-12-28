macro_rules! deps {
    () => {
        RuntimeCombinedLateLintPass!();
        LateContext!();
    };
}

macro_rules! late_lint_mod {
    () => {
        deps!();
        pub fn late_lint_mod < 'tcx , T : LateLintPass < 'tcx > + 'tcx > (tcx : TyCtxt < 'tcx > , module_def_id : LocalModDefId , builtin_lints : T ,) { let context = LateContext { tcx , enclosing_body : None , cached_typeck_results : Cell :: new (None) , param_env : ty :: ParamEnv :: empty () , effective_visibilities : tcx . effective_visibilities (()) , last_node_with_lint_attrs : tcx . local_def_id_to_hir_id (module_def_id) , generics : None , only_module : true , } ; let store = unerased_lint_store (tcx . sess) ; if store . late_module_passes . is_empty () { let dont_need_to_run = tcx . lints_that_dont_need_to_run (()) ; let can_skip_lints = builtin_lints . get_lints () . iter () . all (| lint | dont_need_to_run . contains (& LintId :: of (lint))) ; if ! can_skip_lints { late_lint_mod_inner (tcx , module_def_id , context , builtin_lints) ; } } else { let builtin_lints = Box :: new (builtin_lints) as Box < dyn LateLintPass < 'tcx > > ; let mut binding = store . late_module_passes . iter () . map (| mk_pass | (mk_pass) (tcx)) . chain (std :: iter :: once (builtin_lints)) . collect :: < Vec < _ > > () ; let pass = RuntimeCombinedLateLintPass { passes : binding . as_mut_slice () } ; late_lint_mod_inner (tcx , module_def_id , context , pass) ; } }
    };
}

late_lint_mod!();