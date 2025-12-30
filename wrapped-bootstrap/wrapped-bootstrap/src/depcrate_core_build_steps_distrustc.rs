// Generated macro for Rustc (struct)
macro_rules! Depcrate_core_build_steps_distRustc {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"Rustc"}
// Dependencies: {}
# [doc = " Creates the `rustc` installer component."] # [doc = ""] # [doc = " This includes:"] # [doc = " - The compiler and LLVM."] # [doc = " - Debugger scripts."] # [doc = " - Various helper tools, e.g. LLD or Rust Analyzer proc-macro server (if enabled)."] # [doc = " - The licenses of all code used by the compiler."] # [doc = ""] # [doc = " It does not include any standard library."] # [derive (Debug , Clone , Hash , PartialEq , Eq)] pub struct Rustc { # [doc = " This is the compiler that we will *ship* in this dist step."] pub target_compiler : Compiler , }
};
}
