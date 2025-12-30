// Generated macro for DiagnosticsMapConfig (struct)
macro_rules! Depcrate_cli_diagnosticsDiagnosticsMapConfig {
() => {
// Module: crate::cli::diagnostics
// Provides: {"DiagnosticsMapConfig"}
// Dependencies: {}
# [derive (Debug , Default , Clone)] pub struct DiagnosticsMapConfig { pub remap_prefix : FxHashMap < String , String > , pub warnings_as_info : Vec < String > , pub warnings_as_hint : Vec < String > , pub check_ignore : FxHashSet < String > , }
};
}
