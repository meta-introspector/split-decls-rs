// Generated macro for exported_symbols_for_proc_macro_crate (function)
macro_rules! Depcrate_back_linkerexported_symbols_for_proc_macro_crate {
() => {
// Module: crate::back::linker
// Provides: {"exported_symbols_for_proc_macro_crate"}
// Dependencies: {}
fn exported_symbols_for_proc_macro_crate (tcx : TyCtxt < '_ >) -> Vec < (String , SymbolExportKind) > { if ! tcx . sess . opts . output_types . should_codegen () { return Vec :: new () ; } let stable_crate_id = tcx . stable_crate_id (LOCAL_CRATE) ; let proc_macro_decls_name = tcx . sess . generate_proc_macro_decls_symbol (stable_crate_id) ; vec ! [(proc_macro_decls_name , SymbolExportKind :: Data)] }
};
}
