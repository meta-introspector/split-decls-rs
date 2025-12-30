// Generated macro for is_unreachable_local_definition_provider (function)
macro_rules! Depcrate_back_symbol_exportis_unreachable_local_definition_provider {
() => {
// Module: crate::back::symbol_export
// Provides: {"is_unreachable_local_definition_provider"}
// Dependencies: {}
fn is_unreachable_local_definition_provider (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> bool { ! tcx . reachable_set (()) . contains (& def_id) }
};
}
