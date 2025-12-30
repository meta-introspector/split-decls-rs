// Generated macro for impl_12 (impl)
macro_rules! Depcrate_analysisimpl_12 {
() => {
// Module: crate::analysis
// Provides: {"impl_12"}
// Dependencies: {}
# [cfg (test)] impl AnalysisResults { pub fn query_data_by_label (& self , label : & str) -> & QueryData { self . query_data . iter () . find (| qd | qd . label == label) . unwrap () } pub fn artifact_size_by_label (& self , label : & str) -> & ArtifactSize { self . artifact_sizes . iter () . find (| qd | qd . label == label) . unwrap () } }
};
}
