// Generated macro for is_reachable_non_generic_provider_extern (function)
macro_rules! Depcrate_back_symbol_exportis_reachable_non_generic_provider_extern {
() => {
// Module: crate::back::symbol_export
// Provides: {"is_reachable_non_generic_provider_extern"}
// Dependencies: {}
fn is_reachable_non_generic_provider_extern (tcx : TyCtxt < '_ > , def_id : DefId) -> bool { tcx . reachable_non_generics (def_id . krate) . contains_key (& def_id) }
};
}
