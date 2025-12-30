// Generated macro for exported_symbols_for_non_proc_macro (function)
macro_rules! Depcrate_back_linkerexported_symbols_for_non_proc_macro {
() => {
// Module: crate::back::linker
// Provides: {"exported_symbols_for_non_proc_macro"}
// Dependencies: {}
fn exported_symbols_for_non_proc_macro (tcx : TyCtxt < '_ > , crate_type : CrateType ,) -> Vec < (String , SymbolExportKind) > { let mut symbols = Vec :: new () ; let export_threshold = symbol_export :: crates_export_threshold (& [crate_type]) ; for_each_exported_symbols_include_dep (tcx , crate_type , | symbol , info , cnum | { if info . level . is_below_threshold (export_threshold) && ! tcx . is_compiler_builtins (cnum) { symbols . push ((symbol_export :: exporting_symbol_name_for_instance_in_crate (tcx , symbol , cnum) , info . kind ,)) ; symbol_export :: extend_exported_symbols (& mut symbols , tcx , symbol , cnum) ; } }) ; if export_threshold == SymbolExportLevel :: Rust && needs_allocator_shim_for_linking (tcx . dependency_formats (()) , crate_type) && tcx . allocator_kind (()) . is_some () { symbols . extend (allocator_shim_symbols (tcx)) ; } symbols }
};
}
