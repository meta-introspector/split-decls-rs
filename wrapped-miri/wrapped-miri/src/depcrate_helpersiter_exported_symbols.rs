// Generated macro for iter_exported_symbols (function)
macro_rules! Depcrate_helpersiter_exported_symbols {
() => {
// Module: crate::helpers
// Provides: {"iter_exported_symbols"}
// Dependencies: {}
# [doc = " Call `f` for each exported symbol."] pub fn iter_exported_symbols < 'tcx > (tcx : TyCtxt < 'tcx > , mut f : impl FnMut (CrateNum , DefId) -> InterpResult < 'tcx > ,) -> InterpResult < 'tcx > { let crate_items = tcx . hir_crate_items (()) ; for def_id in crate_items . definitions () { let exported = tcx . def_kind (def_id) . has_codegen_attrs () && { let codegen_attrs = tcx . codegen_fn_attrs (def_id) ; codegen_attrs . contains_extern_indicator () || codegen_attrs . flags . contains (CodegenFnAttrFlags :: USED_COMPILER) || codegen_attrs . flags . contains (CodegenFnAttrFlags :: USED_LINKER) } ; if exported { f (LOCAL_CRATE , def_id . into ()) ? ; } } let dependency_formats = tcx . dependency_formats (()) ; let dependency_format = dependency_formats . get (& CrateType :: Executable) . expect ("interpreting a non-executable crate") ; for cnum in dependency_format . iter_enumerated () . filter_map (| (num , & linkage) | (linkage != Linkage :: NotLinked) . then_some (num)) { if cnum == LOCAL_CRATE { continue ; } for & (symbol , _export_info) in tcx . exported_non_generic_symbols (cnum) { if let ExportedSymbol :: NonGeneric (def_id) = symbol { f (cnum , def_id) ? ; } } } interp_ok (()) }
};
}
