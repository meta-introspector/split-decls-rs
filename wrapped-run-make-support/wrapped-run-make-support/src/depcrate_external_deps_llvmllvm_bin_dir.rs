// Generated macro for llvm_bin_dir (function)
macro_rules! Depcrate_external_deps_llvmllvm_bin_dir {
() => {
// Module: crate::external_deps::llvm
// Provides: {"llvm_bin_dir"}
// Dependencies: {}
# [doc = " Generate the path to the bin directory of LLVM."] # [must_use] pub fn llvm_bin_dir () -> PathBuf { let llvm_bin_dir = env_var ("LLVM_BIN_DIR") ; PathBuf :: from (llvm_bin_dir) }
};
}
