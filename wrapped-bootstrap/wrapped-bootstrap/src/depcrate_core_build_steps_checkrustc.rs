// Generated macro for Rustc (struct)
macro_rules! Depcrate_core_build_steps_checkRustc {
() => {
// Module: crate::core::build_steps::check
// Provides: {"Rustc"}
// Dependencies: {}
# [doc = " Checks rustc using `build_compiler`."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct Rustc { # [doc = " Compiler that will check this rustc."] pub build_compiler : CompilerForCheck , pub target : TargetSelection , # [doc = " Whether to build only a subset of crates."] # [doc = ""] # [doc = " This shouldn't be used from other steps; see the comment on [`compile::Rustc`]."] # [doc = ""] # [doc = " [`compile::Rustc`]: crate::core::build_steps::compile::Rustc"] crates : Vec < String > , }
};
}
