// Generated macro for LlvmProfdata (enum)
macro_rules! Depcrate_trainingLlvmProfdata {
() => {
// Module: crate::training
// Provides: {"LlvmProfdata"}
// Dependencies: {}
# [doc = " Describes which `llvm-profdata` binary should be used for merging PGO profiles."] enum LlvmProfdata { # [doc = " Use llvm-profdata from the host toolchain (i.e. from LLVM provided externally)."] Host , # [doc = " Use llvm-profdata from the target toolchain (i.e. from LLVM built from `src/llvm-project`)."] Target , }
};
}
