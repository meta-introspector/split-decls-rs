// Generated macro for CargoExtractionPhase (enum)
macro_rules! Depcrate_cargo2hf_extractorCargoExtractionPhase {
() => {
// Module: crate::cargo2hf_extractor
// Provides: {"CargoExtractionPhase"}
// Dependencies: {}
# [doc = " Represents different types of data extraction phases for Cargo projects"] # [derive (Debug , Clone , PartialEq , Eq , Hash , Serialize , Deserialize)] pub enum CargoExtractionPhase { # [doc = " Extract basic project metadata from Cargo.toml"] ProjectMetadata , # [doc = " Analyze dependency graph and constraints"] DependencyAnalysis , # [doc = " Extract source code metrics and structure"] SourceCodeAnalysis , # [doc = " Analyze build configuration and scripts"] BuildAnalysis , # [doc = " Extract ecosystem and crates.io metadata"] EcosystemAnalysis , # [doc = " Analyze git history and development patterns"] VersionHistory , }
};
}
