// Generated macro for TypeAssignmentInfo (struct)
macro_rules! Depcrate_rust_analyzer_extractorTypeAssignmentInfo {
() => {
// Module: crate::rust_analyzer_extractor
// Provides: {"TypeAssignmentInfo"}
// Dependencies: {}
# [doc = " Information about type assignments"] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TypeAssignmentInfo { pub location : LocationInfo , pub expression : String , pub inferred_type : String , pub confidence : f32 , }
};
}
