// Generated macro for AnalysisResults (struct)
macro_rules! Depcrate_analysisAnalysisResults {
() => {
// Module: crate::analysis
// Provides: {"AnalysisResults"}
// Dependencies: {}
# [doc = " A collection data for an entire rustc invocation"] # [derive (Serialize , Deserialize , Clone)] pub struct AnalysisResults { pub query_data : Vec < QueryData > , pub artifact_sizes : Vec < ArtifactSize > , pub total_time : Duration , }
};
}
