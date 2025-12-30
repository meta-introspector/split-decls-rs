// Generated macro for ToolBuild (struct)
macro_rules! Depcrate_core_build_steps_toolToolBuild {
() => {
// Module: crate::core::build_steps::tool
// Provides: {"ToolBuild"}
// Dependencies: {}
# [derive (Debug , Clone , Hash , PartialEq , Eq)] struct ToolBuild { # [doc = " Compiler that will build this tool."] build_compiler : Compiler , target : TargetSelection , tool : & 'static str , path : & 'static str , mode : Mode , source_type : SourceType , extra_features : Vec < String > , # [doc = " Nightly-only features that are allowed (comma-separated list)."] allow_features : & 'static str , # [doc = " Additional arguments to pass to the `cargo` invocation."] cargo_args : Vec < String > , # [doc = " Whether the tool builds a binary or a library."] artifact_kind : ToolArtifactKind , }
};
}
