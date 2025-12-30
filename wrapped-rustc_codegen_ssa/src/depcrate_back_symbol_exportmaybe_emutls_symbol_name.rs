// Generated macro for maybe_emutls_symbol_name (function)
macro_rules! Depcrate_back_symbol_exportmaybe_emutls_symbol_name {
() => {
// Module: crate::back::symbol_export
// Provides: {"maybe_emutls_symbol_name"}
// Dependencies: {}
fn maybe_emutls_symbol_name < 'tcx > (tcx : TyCtxt < 'tcx > , symbol : ExportedSymbol < 'tcx > , undecorated : & str ,) -> Option < String > { if matches ! (tcx . sess . tls_model () , TlsModel :: Emulated) && let ExportedSymbol :: NonGeneric (def_id) = symbol && tcx . is_thread_local_static (def_id) { Some (format ! ("__emutls_v.{undecorated}")) } else { None } }
};
}
