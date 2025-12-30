// Generated macro for ParseErrorInfo (struct)
macro_rules! Depcrate_rust_analyzer_extractorParseErrorInfo {
() => {
// Module: crate::rust_analyzer_extractor
// Provides: {"ParseErrorInfo"}
// Dependencies: {}
# [doc = " Information about parse errors"] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ParseErrorInfo { pub message : String , pub start : u32 , pub end : u32 , pub severity : String , }
};
}
