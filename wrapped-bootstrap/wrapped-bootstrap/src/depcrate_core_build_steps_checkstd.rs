// Generated macro for Std (struct)
macro_rules! Depcrate_core_build_steps_checkStd {
() => {
// Module: crate::core::build_steps::check
// Provides: {"Std"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct Std { # [doc = " Compiler that will check this std."] pub build_compiler : Compiler , pub target : TargetSelection , # [doc = " Whether to build only a subset of crates."] # [doc = ""] # [doc = " This shouldn't be used from other steps; see the comment on [`compile::Rustc`]."] # [doc = ""] # [doc = " [`compile::Rustc`]: crate::core::build_steps::compile::Rustc"] crates : Vec < String > , }
};
}
