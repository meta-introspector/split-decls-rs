// Generated macro for wasm_import_module (function)
macro_rules! Depcrate_attributeswasm_import_module {
() => {
// Module: crate::attributes
// Provides: {"wasm_import_module"}
// Dependencies: {}
fn wasm_import_module (tcx : TyCtxt < '_ > , id : DefId) -> Option < & String > { tcx . wasm_import_module_map (id . krate) . get (& id) }
};
}
