// Generated macro for Rustdoc (struct)
macro_rules! Depcrate_core_build_steps_toolRustdoc {
() => {
// Module: crate::core::build_steps::tool
// Provides: {"Rustdoc"}
// Dependencies: {}
# [doc = " Represents `Rustdoc` that either comes from the external stage0 sysroot or that is built"] # [doc = " locally."] # [doc = " Rustdoc is special, because it both essentially corresponds to a `Compiler` (that can be"] # [doc = " externally provided), but also to a `ToolRustcPrivate` tool."] # [derive (Debug , Clone , Hash , PartialEq , Eq)] pub struct Rustdoc { # [doc = " If the stage of `target_compiler` is `0`, then rustdoc is externally provided."] # [doc = " Otherwise it is built locally."] pub target_compiler : Compiler , }
};
}
