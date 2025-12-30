// Generated macro for ParsingPhaseData (struct)
macro_rules! Depcrate_rust_analyzer_extractorParsingPhaseData {
() => {
// Module: crate::rust_analyzer_extractor
// Provides: {"ParsingPhaseData"}
// Dependencies: {}
# [doc = " Data captured during the parsing phase"] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ParsingPhaseData { pub file_path : String , pub source_code : String , pub syntax_tree_json : String , pub tokens : Vec < TokenInfo > , pub parse_errors : Vec < ParseErrorInfo > , pub parse_time_ms : u64 , }
};
}
