// Generated macro for used_trait_imports (function)
macro_rules! Depcrateused_trait_imports {
() => {
// Module: crate
// Provides: {"used_trait_imports"}
// Dependencies: {}
fn used_trait_imports (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> & UnordSet < LocalDefId > { & tcx . typeck (def_id) . used_trait_imports }
};
}
