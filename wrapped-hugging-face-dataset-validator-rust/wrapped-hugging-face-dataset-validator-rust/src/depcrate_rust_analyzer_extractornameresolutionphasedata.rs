// Generated macro for NameResolutionPhaseData (struct)
macro_rules! Depcrate_rust_analyzer_extractorNameResolutionPhaseData {
() => {
// Module: crate::rust_analyzer_extractor
// Provides: {"NameResolutionPhaseData"}
// Dependencies: {}
# [doc = " Data captured during the name resolution phase"] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct NameResolutionPhaseData { pub file_path : String , pub symbols : Vec < SymbolInfo > , pub scopes : Vec < ScopeInfo > , pub imports : Vec < ImportInfo > , pub unresolved_names : Vec < UnresolvedNameInfo > , pub resolution_time_ms : u64 , }
};
}
