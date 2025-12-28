macro_rules! symbol_export_level {
    () => {
        fn symbol_export_level (tcx : TyCtxt < '_ > , sym_def_id : DefId) -> SymbolExportLevel { let codegen_fn_attrs = tcx . codegen_fn_attrs (sym_def_id) ; let is_extern = codegen_fn_attrs . contains_extern_indicator () ; let std_internal = codegen_fn_attrs . flags . contains (CodegenFnAttrFlags :: RUSTC_STD_INTERNAL_SYMBOL) ; if is_extern && ! std_internal { let target = & tcx . sess . target . llvm_target ; if target . contains ("emscripten") { if let DefKind :: Static { .. } = tcx . def_kind (sym_def_id) { return SymbolExportLevel :: Rust ; } } SymbolExportLevel :: C } else { SymbolExportLevel :: Rust } }
    };
}

symbol_export_level!();