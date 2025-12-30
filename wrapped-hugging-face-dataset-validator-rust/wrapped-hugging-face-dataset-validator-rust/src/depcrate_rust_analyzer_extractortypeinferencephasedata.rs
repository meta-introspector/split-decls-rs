// Generated macro for TypeInferencePhaseData (struct)
macro_rules! Depcrate_rust_analyzer_extractorTypeInferencePhaseData {
() => {
// Module: crate::rust_analyzer_extractor
// Provides: {"TypeInferencePhaseData"}
// Dependencies: {}
# [doc = " Data captured during the type inference phase"] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct TypeInferencePhaseData { pub file_path : String , pub type_assignments : Vec < TypeAssignmentInfo > , pub type_errors : Vec < TypeErrorInfo > , pub inferred_types : Vec < InferredTypeInfo > , pub inference_time_ms : u64 , }
};
}
