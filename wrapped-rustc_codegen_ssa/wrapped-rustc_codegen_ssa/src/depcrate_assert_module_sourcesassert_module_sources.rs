// Generated macro for assert_module_sources (function)
macro_rules! Depcrate_assert_module_sourcesassert_module_sources {
() => {
// Module: crate::assert_module_sources
// Provides: {"assert_module_sources"}
// Dependencies: {}
# [allow (missing_docs)] pub fn assert_module_sources (tcx : TyCtxt < '_ > , set_reuse : & dyn Fn (& mut CguReuseTracker)) { tcx . dep_graph . with_ignore (| | { if tcx . sess . opts . incremental . is_none () { return ; } let available_cgus = tcx . collect_and_partition_mono_items (()) . codegen_units . iter () . map (| cgu | cgu . name ()) . collect () ; let mut ams = AssertModuleSource { tcx , available_cgus , cgu_reuse_tracker : if tcx . sess . opts . unstable_opts . query_dep_graph { CguReuseTracker :: new () } else { CguReuseTracker :: new_disabled () } , } ; for attr in tcx . hir_attrs (rustc_hir :: CRATE_HIR_ID) { ams . check_attr (attr) ; } set_reuse (& mut ams . cgu_reuse_tracker) ; if tcx . sess . opts . unstable_opts . print_mono_items && let Some (data) = & ams . cgu_reuse_tracker . data { data . actual_reuse . items () . all (| (cgu , reuse) | { println ! ("CGU_REUSE {cgu} {reuse}") ; true }) ; } ams . cgu_reuse_tracker . check_expected_reuse (tcx . sess) ; }) ; }
};
}
