// Generated macro for is_reachable_non_generic_provider_local (function)
macro_rules! Depcrate_back_symbol_exportis_reachable_non_generic_provider_local {
() => {
// Module: crate::back::symbol_export
// Provides: {"is_reachable_non_generic_provider_local"}
// Dependencies: {}
fn is_reachable_non_generic_provider_local (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> bool { let export_threshold = threshold (tcx) ; if let Some (& info) = tcx . reachable_non_generics (LOCAL_CRATE) . get (& def_id . to_def_id ()) { info . level . is_below_threshold (export_threshold) } else { false } }
};
}
