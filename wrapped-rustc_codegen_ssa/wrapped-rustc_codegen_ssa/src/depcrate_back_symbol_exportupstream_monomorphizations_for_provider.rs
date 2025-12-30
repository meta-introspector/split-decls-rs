// Generated macro for upstream_monomorphizations_for_provider (function)
macro_rules! Depcrate_back_symbol_exportupstream_monomorphizations_for_provider {
() => {
// Module: crate::back::symbol_export
// Provides: {"upstream_monomorphizations_for_provider"}
// Dependencies: {}
fn upstream_monomorphizations_for_provider (tcx : TyCtxt < '_ > , def_id : DefId ,) -> Option < & UnordMap < GenericArgsRef < '_ > , CrateNum > > { assert ! (! def_id . is_local ()) ; tcx . upstream_monomorphizations (()) . get (& def_id) }
};
}
