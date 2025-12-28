macro_rules! allocator_shim_symbols {
    () => {
        pub (crate) fn allocator_shim_symbols (tcx : TyCtxt < '_ > ,) -> impl Iterator < Item = (String , SymbolExportKind) > { ALLOCATOR_METHODS . iter () . map (move | method | mangle_internal_symbol (tcx , global_fn_name (method . name) . as_str ())) . chain ([mangle_internal_symbol (tcx , "__rust_alloc_error_handler") , mangle_internal_symbol (tcx , OomStrategy :: SYMBOL) , mangle_internal_symbol (tcx , NO_ALLOC_SHIM_IS_UNSTABLE) ,]) . map (move | symbol_name | { let exported_symbol = ExportedSymbol :: NoDefId (SymbolName :: new (tcx , & symbol_name)) ; (symbol_export :: exporting_symbol_name_for_instance_in_crate (tcx , exported_symbol , LOCAL_CRATE ,) , SymbolExportKind :: Text ,) }) }
    };
}

allocator_shim_symbols!()