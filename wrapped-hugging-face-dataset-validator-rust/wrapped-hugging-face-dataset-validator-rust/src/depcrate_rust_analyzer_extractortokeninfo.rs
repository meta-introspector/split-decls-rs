// Generated macro for TokenInfo (struct)
macro_rules! Depcrate_rust_analyzer_extractorTokenInfo {
() => {
// Module: crate::rust_analyzer_extractor
// Provides: {"TokenInfo"}
// Dependencies: {}
# [doc = " Information about individual tokens"] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TokenInfo { pub kind : String , pub text : String , pub start : u32 , pub end : u32 , pub line : u32 , pub column : u32 , }
};
}
