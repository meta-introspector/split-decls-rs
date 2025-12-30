// Generated macro for impl_10795 (impl)
macro_rules! Depcrate_unused_trait_namesimpl_10795 {
() => {
// Module: crate::unused_trait_names
// Provides: {"impl_10795"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for UnusedTraitNames { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx Item < 'tcx >) { if ! item . span . from_expansion () && let ItemKind :: Use (path , UseKind :: Single (ident)) = item . kind && ident . name != kw :: Underscore && let Some (Res :: Def (DefKind :: Trait , _)) = path . res . type_ns && cx . tcx . resolutions (()) . maybe_unused_trait_imports . contains (& item . owner_id . def_id) && let module = cx . tcx . parent_module_from_def_id (item . owner_id . def_id) && cx . tcx . visibility (item . owner_id . def_id) == Visibility :: Restricted (module . to_def_id ()) && let Some (last_segment) = path . segments . last () && let Some (snip) = snippet_opt (cx , last_segment . ident . span) && self . msrv . meets (cx , msrvs :: UNDERSCORE_IMPORTS) && ! is_from_proc_macro (cx , & last_segment . ident) { let complete_span = last_segment . ident . span . to (ident . span) ; span_lint_and_sugg (cx , UNUSED_TRAIT_NAMES , complete_span , "importing trait that is only used anonymously" , "use" , format ! ("{snip} as _") , Applicability :: MachineApplicable ,) ; } } }
};
}
