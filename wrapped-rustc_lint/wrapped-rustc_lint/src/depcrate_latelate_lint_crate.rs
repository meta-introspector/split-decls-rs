// Generated macro for late_lint_crate (function)
macro_rules! Depcrate_latelate_lint_crate {
() => {
// Module: crate::late
// Provides: {"late_lint_crate"}
// Dependencies: {}
fn late_lint_crate < 'tcx > (tcx : TyCtxt < 'tcx >) { let passes : Vec < _ > = unerased_lint_store (tcx . sess) . late_passes . iter () . map (| mk_pass | (mk_pass) (tcx)) . collect () ; if passes . is_empty () { return ; } let context = LateContext { tcx , enclosing_body : None , cached_typeck_results : Cell :: new (None) , param_env : ty :: ParamEnv :: empty () , effective_visibilities : tcx . effective_visibilities (()) , last_node_with_lint_attrs : hir :: CRATE_HIR_ID , generics : None , only_module : false , } ; let lints_that_dont_need_to_run = tcx . lints_that_dont_need_to_run (()) ; let mut filtered_passes : Vec < Box < dyn LateLintPass < 'tcx > > > = passes . into_iter () . filter (| pass | { let lints = (* * pass) . get_lints () ; lints . is_empty () || ! lints . iter () . all (| lint | lints_that_dont_need_to_run . contains (& LintId :: of (lint))) }) . collect () ; filtered_passes . push (Box :: new (HardwiredLints)) ; let pass = RuntimeCombinedLateLintPass { passes : & mut filtered_passes [..] } ; late_lint_crate_inner (tcx , context , pass) ; }
};
}
