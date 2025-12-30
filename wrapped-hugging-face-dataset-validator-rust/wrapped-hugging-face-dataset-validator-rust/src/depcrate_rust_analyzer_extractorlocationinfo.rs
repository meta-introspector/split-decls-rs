// Generated macro for LocationInfo (struct)
macro_rules! Depcrate_rust_analyzer_extractorLocationInfo {
() => {
// Module: crate::rust_analyzer_extractor
// Provides: {"LocationInfo"}
// Dependencies: {}
# [doc = " Generic location information"] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct LocationInfo { pub file_path : String , pub line : u32 , pub column : u32 , pub start : u32 , pub end : u32 , }
};
}
