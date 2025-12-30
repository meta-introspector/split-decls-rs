// Generated macro for LlvmResult (struct)
macro_rules! Depcrate_core_build_steps_llvmLlvmResult {
() => {
// Module: crate::core::build_steps::llvm
// Provides: {"LlvmResult"}
// Dependencies: {}
# [derive (Clone)] pub struct LlvmResult { # [doc = " Path to llvm-config binary."] # [doc = " NB: This is always the host llvm-config!"] pub host_llvm_config : PathBuf , # [doc = " Path to LLVM cmake directory for the target."] pub llvm_cmake_dir : PathBuf , }
};
}
