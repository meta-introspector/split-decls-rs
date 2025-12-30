// Generated macro for variances_of (function)
macro_rules! Depcrate_variancevariances_of {
() => {
// Module: crate::variance
// Provides: {"variances_of"}
// Dependencies: {}
pub (super) fn variances_of (tcx : TyCtxt < '_ > , item_def_id : LocalDefId) -> & [ty :: Variance] { if tcx . generics_of (item_def_id) . is_empty () { return & [] ; } let kind = tcx . def_kind (item_def_id) ; match kind { DefKind :: Fn | DefKind :: AssocFn | DefKind :: Enum | DefKind :: Struct | DefKind :: Union | DefKind :: Ctor (..) => { let crate_map = tcx . crate_variances (()) ; return crate_map . variances . get (& item_def_id . to_def_id ()) . copied () . unwrap_or (& []) ; } DefKind :: TyAlias if tcx . type_alias_is_lazy (item_def_id) => { let crate_map = tcx . crate_variances (()) ; return crate_map . variances . get (& item_def_id . to_def_id ()) . copied () . unwrap_or (& []) ; } DefKind :: AssocTy => match tcx . opt_rpitit_info (item_def_id . to_def_id ()) { Some (ty :: ImplTraitInTraitData :: Trait { opaque_def_id , .. }) => { return variance_of_opaque (tcx , opaque_def_id . expect_local () , ForceCaptureTraitArgs :: Yes ,) ; } None | Some (ty :: ImplTraitInTraitData :: Impl { .. }) => { } } , DefKind :: OpaqueTy => { let force_capture_trait_args = if let hir :: OpaqueTyOrigin :: FnReturn { parent : _ , in_trait_or_impl : Some (hir :: RpitContext :: Trait) , } = tcx . hir_node_by_def_id (item_def_id) . expect_opaque_ty () . origin { ForceCaptureTraitArgs :: Yes } else { ForceCaptureTraitArgs :: No } ; return variance_of_opaque (tcx , item_def_id , force_capture_trait_args) ; } _ => { } } span_bug ! (tcx . def_span (item_def_id) , "asked to compute variance for {}" , kind . descr (item_def_id . to_def_id ())) ; }
};
}
