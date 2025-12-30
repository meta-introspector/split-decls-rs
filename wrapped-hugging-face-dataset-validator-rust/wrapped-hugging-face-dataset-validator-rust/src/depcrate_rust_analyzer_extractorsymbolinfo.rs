// Generated macro for SymbolInfo (struct)
macro_rules! Depcrate_rust_analyzer_extractorSymbolInfo {
() => {
// Module: crate::rust_analyzer_extractor
// Provides: {"SymbolInfo"}
// Dependencies: {}
# [doc = " Information about resolved symbols"] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct SymbolInfo { pub name : String , pub kind : String , pub definition_location : LocationInfo , pub visibility : String , pub signature : Option < String > , }
};
}
