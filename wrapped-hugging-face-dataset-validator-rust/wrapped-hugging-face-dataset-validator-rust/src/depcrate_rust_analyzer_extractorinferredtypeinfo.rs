// Generated macro for InferredTypeInfo (struct)
macro_rules! Depcrate_rust_analyzer_extractorInferredTypeInfo {
() => {
// Module: crate::rust_analyzer_extractor
// Provides: {"InferredTypeInfo"}
// Dependencies: {}
# [doc = " Information about inferred types"] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct InferredTypeInfo { pub symbol_name : String , pub location : LocationInfo , pub inferred_type : String , pub inference_method : String , }
};
}
