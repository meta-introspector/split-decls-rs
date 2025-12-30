// Generated macro for SanitizerRuntime (struct)
macro_rules! Depcrate_core_build_steps_llvmSanitizerRuntime {
() => {
// Module: crate::core::build_steps::llvm
// Provides: {"SanitizerRuntime"}
// Dependencies: {}
# [derive (Clone , Debug)] pub struct SanitizerRuntime { # [doc = " CMake target used to build the runtime."] pub cmake_target : String , # [doc = " Path to the built runtime library."] pub path : PathBuf , # [doc = " Library filename that will be used rustc."] pub name : String , }
};
}
