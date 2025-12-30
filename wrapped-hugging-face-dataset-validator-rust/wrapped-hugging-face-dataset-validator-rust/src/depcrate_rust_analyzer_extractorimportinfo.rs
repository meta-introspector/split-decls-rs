// Generated macro for ImportInfo (struct)
macro_rules! Depcrate_rust_analyzer_extractorImportInfo {
() => {
// Module: crate::rust_analyzer_extractor
// Provides: {"ImportInfo"}
// Dependencies: {}
# [doc = " Information about import statements"] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ImportInfo { pub path : String , pub alias : Option < String > , pub location : LocationInfo , pub resolved : bool , }
};
}
