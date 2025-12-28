macro_rules! exported_symbols_for_proc_macro_crate {
    () => {
        fn exported_symbols_for_proc_macro_crate (tcx : TyCtxt < '_ >) -> Vec < (String , SymbolExportKind) > { if ! tcx . sess . opts . output_types . should_codegen () { return Vec :: new () ; } let stable_crate_id = tcx . stable_crate_id (LOCAL_CRATE) ; let proc_macro_decls_name = tcx . sess . generate_proc_macro_decls_symbol (stable_crate_id) ; vec ! [(proc_macro_decls_name , SymbolExportKind :: Data)] }
    };
}

exported_symbols_for_proc_macro_crate!();