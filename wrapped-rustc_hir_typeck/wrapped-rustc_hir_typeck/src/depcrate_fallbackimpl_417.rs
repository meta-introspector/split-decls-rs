// Generated macro for impl_417 (impl)
macro_rules! Depcrate_fallbackimpl_417 {
() => {
// Module: crate::fallback
// Provides: {"impl_417"}
// Dependencies: {}
impl < 'tcx > AnnotateUnitFallbackVisitor < '_ , 'tcx > { fn suggest_for_segment (& self , arg_segment : & 'tcx hir :: PathSegment < 'tcx > , def_id : DefId , id : HirId ,) -> ControlFlow < errors :: SuggestAnnotation > { if arg_segment . args . is_none () && let Some (all_args) = self . fcx . typeck_results . borrow () . node_args_opt (id) && let generics = self . fcx . tcx . generics_of (def_id) && let args = all_args [generics . parent_count ..] . iter () . zip (& generics . own_params) && args . clone () . all (| (_ , param) | matches ! (param . kind , ty :: GenericParamDefKind :: Type { .. } | ty :: GenericParamDefKind :: Lifetime)) { let non_apit_type_args = args . filter (| (_ , param) | { matches ! (param . kind , ty :: GenericParamDefKind :: Type { synthetic : false , .. }) }) ; let n_tys = non_apit_type_args . clone () . count () ; for (idx , (arg , _)) in non_apit_type_args . enumerate () { if let Some (ty) = arg . as_type () && let Some (vid) = self . fcx . root_vid (ty) && self . reachable_vids . contains (& vid) { return ControlFlow :: Break (errors :: SuggestAnnotation :: Turbo (arg_segment . ident . span . shrink_to_hi () , n_tys , idx ,)) ; } } } ControlFlow :: Continue (()) } }
};
}
