// Generated macro for Rustc (struct)
macro_rules! Depcrate_core_build_steps_compileRustc {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"Rustc"}
// Dependencies: {}
# [doc = " Build rustc using the passed `build_compiler`."] # [doc = ""] # [doc = " - Makes sure that `build_compiler` has a standard library prepared for its host target,"] # [doc = "   so that it can compile build scripts and proc macros when building this `rustc`."] # [doc = " - Makes sure that `build_compiler` has a standard library prepared for `target`,"] # [doc = "   so that the built `rustc` can *link to it* and use it at runtime."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct Rustc { # [doc = " The target on which rustc will run (its host)."] pub target : TargetSelection , # [doc = " The **previous** compiler used to compile this rustc."] pub build_compiler : Compiler , # [doc = " Whether to build a subset of crates, rather than the whole compiler."] # [doc = ""] # [doc = " This should only be requested by the user, not used within bootstrap itself."] # [doc = " Using it within bootstrap can lead to confusing situation where lints are replayed"] # [doc = " in two different steps."] crates : Vec < String > , }
};
}
