// Generated macro for TypeErrorInfo (struct)
macro_rules! Depcrate_rust_analyzer_extractorTypeErrorInfo {
() => {
// Module: crate::rust_analyzer_extractor
// Provides: {"TypeErrorInfo"}
// Dependencies: {}
# [doc = " Information about type errors"] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TypeErrorInfo { pub message : String , pub location : LocationInfo , pub expected_type : Option < String > , pub actual_type : Option < String > , }
};
}
