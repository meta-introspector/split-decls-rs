// Generated macro for crates_export_threshold (function)
macro_rules! Depcrate_back_symbol_exportcrates_export_threshold {
() => {
// Module: crate::back::symbol_export
// Provides: {"crates_export_threshold"}
// Dependencies: {}
pub fn crates_export_threshold (crate_types : & [CrateType]) -> SymbolExportLevel { if crate_types . iter () . any (| & crate_type | crate_export_threshold (crate_type) == SymbolExportLevel :: Rust) { SymbolExportLevel :: Rust } else { SymbolExportLevel :: C } }
};
}
