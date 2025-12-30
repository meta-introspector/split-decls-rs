// Generated macro for exported_non_generic_symbols_provider_local (function)
macro_rules! Depcrate_back_symbol_exportexported_non_generic_symbols_provider_local {
() => {
// Module: crate::back::symbol_export
// Provides: {"exported_non_generic_symbols_provider_local"}
// Dependencies: {}
fn exported_non_generic_symbols_provider_local < 'tcx > (tcx : TyCtxt < 'tcx > , _ : LocalCrate ,) -> & 'tcx [(ExportedSymbol < 'tcx > , SymbolExportInfo)] { if ! tcx . sess . opts . output_types . should_codegen () && ! tcx . is_sdylib_interface_build () { return & [] ; } let sorted = tcx . with_stable_hashing_context (| hcx | { tcx . reachable_non_generics (LOCAL_CRATE) . to_sorted (& hcx , true) }) ; let mut symbols : Vec < _ > = sorted . iter () . map (| & (& def_id , & info) | (ExportedSymbol :: NonGeneric (def_id) , info)) . collect () ; if ! tcx . sess . target . dll_tls_export { symbols . extend (sorted . iter () . filter_map (| & (& def_id , & info) | { tcx . needs_thread_local_shim (def_id) . then (| | { (ExportedSymbol :: ThreadLocalShim (def_id) , SymbolExportInfo { level : info . level , kind : SymbolExportKind :: Text , used : info . used , rustc_std_internal_symbol : info . rustc_std_internal_symbol , } ,) }) })) } if tcx . entry_fn (()) . is_some () { let exported_symbol = ExportedSymbol :: NoDefId (SymbolName :: new (tcx , tcx . sess . target . entry_name . as_ref ())) ; symbols . push ((exported_symbol , SymbolExportInfo { level : SymbolExportLevel :: C , kind : SymbolExportKind :: Text , used : false , rustc_std_internal_symbol : false , } ,)) ; } symbols . sort_by_cached_key (| s | s . 0 . symbol_name_for_local_instance (tcx)) ; tcx . arena . alloc_from_iter (symbols) }
};
}
