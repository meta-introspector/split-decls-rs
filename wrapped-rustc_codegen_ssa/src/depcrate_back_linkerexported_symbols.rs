// Generated macro for exported_symbols (function)
macro_rules! Depcrate_back_linkerexported_symbols {
() => {
// Module: crate::back::linker
// Provides: {"exported_symbols"}
// Dependencies: {}
pub (crate) fn exported_symbols (tcx : TyCtxt < '_ > , crate_type : CrateType ,) -> Vec < (String , SymbolExportKind) > { if let Some (ref exports) = tcx . sess . target . override_export_symbols { return exports . iter () . map (| name | { (name . to_string () , SymbolExportKind :: Text ,) }) . collect () ; } let mut symbols = if let CrateType :: ProcMacro = crate_type { exported_symbols_for_proc_macro_crate (tcx) } else { exported_symbols_for_non_proc_macro (tcx , crate_type) } ; if crate_type == CrateType :: Dylib || crate_type == CrateType :: ProcMacro { let metadata_symbol_name = exported_symbols :: metadata_symbol_name (tcx) ; symbols . push ((metadata_symbol_name , SymbolExportKind :: Data)) ; } symbols }
};
}
