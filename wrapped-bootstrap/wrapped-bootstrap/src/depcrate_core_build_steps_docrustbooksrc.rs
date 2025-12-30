// Generated macro for RustbookSrc (struct)
macro_rules! Depcrate_core_build_steps_docRustbookSrc {
() => {
// Module: crate::core::build_steps::doc
// Provides: {"RustbookSrc"}
// Dependencies: {}
# [derive (Debug , Clone , Hash , PartialEq , Eq)] struct RustbookSrc < P : Step > { target : TargetSelection , name : String , src : PathBuf , parent : Option < P > , languages : Vec < & 'static str > , # [doc = " Compiler whose rustdoc should be used to document things using `mdbook-spec`."] build_compiler : Option < Compiler > , }
};
}
