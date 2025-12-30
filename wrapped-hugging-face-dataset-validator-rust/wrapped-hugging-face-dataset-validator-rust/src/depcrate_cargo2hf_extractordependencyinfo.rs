// Generated macro for DependencyInfo (struct)
macro_rules! Depcrate_cargo2hf_extractorDependencyInfo {
() => {
// Module: crate::cargo2hf_extractor
// Provides: {"DependencyInfo"}
// Dependencies: {}
# [doc = " Detailed dependency information"] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct DependencyInfo { # [doc = " Name of the dependency"] pub name : String , # [doc = " Version requirement (e.g., \"^1.0\", \"=0.2.5\")"] pub version_req : String , # [doc = " Resolved version (if available)"] pub resolved_version : Option < String > , # [doc = " Whether this is an optional dependency"] pub optional : bool , # [doc = " Default features enabled"] pub default_features : bool , # [doc = " Specific features enabled"] pub features : Vec < String > , # [doc = " Dependency source (crates.io, git, path, etc.)"] pub source : String , # [doc = " Whether this is a dev dependency"] pub is_dev : bool , # [doc = " Whether this is a build dependency"] pub is_build : bool , }
};
}
