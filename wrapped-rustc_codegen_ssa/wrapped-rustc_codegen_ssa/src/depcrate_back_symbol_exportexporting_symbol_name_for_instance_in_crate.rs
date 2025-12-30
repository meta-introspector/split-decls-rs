// Generated macro for exporting_symbol_name_for_instance_in_crate (function)
macro_rules! Depcrate_back_symbol_exportexporting_symbol_name_for_instance_in_crate {
() => {
// Module: crate::back::symbol_export
// Provides: {"exporting_symbol_name_for_instance_in_crate"}
// Dependencies: {}
pub (crate) fn exporting_symbol_name_for_instance_in_crate < 'tcx > (tcx : TyCtxt < 'tcx > , symbol : ExportedSymbol < 'tcx > , cnum : CrateNum ,) -> String { let undecorated = symbol_name_for_instance_in_crate (tcx , symbol , cnum) ; maybe_emutls_symbol_name (tcx , symbol , & undecorated) . unwrap_or (undecorated) }
};
}
