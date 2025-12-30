// Generated macro for Rustc (struct)
macro_rules! Depcrate_core_build_steps_clippyRustc {
() => {
// Module: crate::core::build_steps::clippy
// Provides: {"Rustc"}
// Dependencies: {}
# [doc = " Lints the compiler."] # [doc = ""] # [doc = " This will build Clippy with the `build_compiler` and use it to lint"] # [doc = " in-tree rustc."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct Rustc { build_compiler : CompilerForCheck , target : TargetSelection , config : LintConfig , # [doc = " Whether to lint only a subset of crates."] crates : Vec < String > , }
};
}
