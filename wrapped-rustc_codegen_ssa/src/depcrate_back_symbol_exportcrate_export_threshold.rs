// Generated macro for crate_export_threshold (function)
macro_rules! Depcrate_back_symbol_exportcrate_export_threshold {
() => {
// Module: crate::back::symbol_export
// Provides: {"crate_export_threshold"}
// Dependencies: {}
fn crate_export_threshold (crate_type : CrateType) -> SymbolExportLevel { match crate_type { CrateType :: Executable | CrateType :: Staticlib | CrateType :: ProcMacro | CrateType :: Cdylib => { SymbolExportLevel :: C } CrateType :: Rlib | CrateType :: Dylib | CrateType :: Sdylib => SymbolExportLevel :: Rust , } }
};
}
