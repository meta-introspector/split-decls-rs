// Generated macro for llvm_readobj (function)
macro_rules! Depcrate_external_deps_llvmllvm_readobj {
() => {
// Module: crate::external_deps::llvm
// Provides: {"llvm_readobj"}
// Dependencies: {}
# [doc = " Construct a new `llvm-readobj` invocation with the `GNU` output style."] # [doc = " This assumes that `llvm-readobj` is available at `$LLVM_BIN_DIR/llvm-readobj`."] # [track_caller] pub fn llvm_readobj () -> LlvmReadobj { LlvmReadobj :: new () }
};
}
