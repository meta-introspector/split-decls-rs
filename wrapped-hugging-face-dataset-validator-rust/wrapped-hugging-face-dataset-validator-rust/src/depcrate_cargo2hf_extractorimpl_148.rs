// Generated macro for impl_148 (impl)
macro_rules! Depcrate_cargo2hf_extractorimpl_148 {
() => {
// Module: crate::cargo2hf_extractor
// Provides: {"impl_148"}
// Dependencies: {}
impl CargoExtractionPhase { # [doc = " Convert phase to string representation for dataset naming"] pub fn as_str (& self) -> & 'static str { match self { CargoExtractionPhase :: ProjectMetadata => "project_metadata" , CargoExtractionPhase :: DependencyAnalysis => "dependency_analysis" , CargoExtractionPhase :: SourceCodeAnalysis => "source_code_analysis" , CargoExtractionPhase :: BuildAnalysis => "build_analysis" , CargoExtractionPhase :: EcosystemAnalysis => "ecosystem_analysis" , CargoExtractionPhase :: VersionHistory => "version_history" , } } }
};
}
