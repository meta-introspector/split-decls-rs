// Generated macro for ScopeInfo (struct)
macro_rules! Depcrate_rust_analyzer_extractorScopeInfo {
() => {
// Module: crate::rust_analyzer_extractor
// Provides: {"ScopeInfo"}
// Dependencies: {}
# [doc = " Information about lexical scopes"] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ScopeInfo { pub scope_id : String , pub parent_scope : Option < String > , pub start : u32 , pub end : u32 , pub symbols : Vec < String > , }
};
}
