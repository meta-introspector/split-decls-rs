// Generated macro for llvm_benchmarks (function)
macro_rules! Depcrate_trainingllvm_benchmarks {
() => {
// Module: crate::training
// Provides: {"llvm_benchmarks"}
// Dependencies: {}
pub fn llvm_benchmarks (env : & Environment) -> CmdBuilder { init_compiler_benchmarks (env , & ["Debug" , "Opt"] , & ["Full"] , LLVM_PGO_CRATES) }
};
}
