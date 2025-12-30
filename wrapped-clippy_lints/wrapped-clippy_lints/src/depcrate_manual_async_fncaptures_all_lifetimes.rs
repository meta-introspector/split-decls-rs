// Generated macro for captures_all_lifetimes (function)
macro_rules! Depcrate_manual_async_fncaptures_all_lifetimes {
() => {
// Module: crate::manual_async_fn
// Provides: {"captures_all_lifetimes"}
// Dependencies: {}
fn captures_all_lifetimes (cx : & LateContext < '_ > , fn_def_id : LocalDefId , opaque_def_id : LocalDefId) -> bool { let early_input_params = ty :: GenericArgs :: identity_for_item (cx . tcx , fn_def_id) ; let late_input_params = cx . tcx . late_bound_vars (cx . tcx . local_def_id_to_hir_id (fn_def_id)) ; let num_early_lifetimes = early_input_params . iter () . filter (| param | param . as_region () . is_some ()) . count () ; let num_late_lifetimes = late_input_params . iter () . filter (| param_kind | matches ! (param_kind , ty :: BoundVariableKind :: Region (_))) . count () ; if num_early_lifetimes == 0 && num_late_lifetimes == 0 { return true ; } let num_captured_lifetimes = cx . tcx . opaque_captured_lifetimes (opaque_def_id) . iter () . filter (| & (lifetime , _) | { matches ! (* lifetime , ResolvedArg :: EarlyBound (_) | ResolvedArg :: LateBound (ty :: INNERMOST , _ , _)) }) . count () ; num_captured_lifetimes == num_early_lifetimes + num_late_lifetimes }
};
}
