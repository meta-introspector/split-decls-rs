// Generated macro for LLVMAnalysisPhase (enum)
macro_rules! Depcrate_llvm_ir_extractorLLVMAnalysisPhase {
() => {
// Module: crate::llvm_ir_extractor
// Provides: {"LLVMAnalysisPhase"}
// Dependencies: {}
# [doc = " Represents different phases of LLVM IR analysis and generation"] # [derive (Debug , Clone , PartialEq , Eq , Hash , Serialize , Deserialize)] pub enum LLVMAnalysisPhase { # [doc = " Initial IR generation from Rust MIR"] IRGeneration , # [doc = " LLVM optimization passes analysis"] OptimizationPasses , # [doc = " Code generation and target-specific optimizations"] CodeGeneration , # [doc = " Performance analysis and correlation"] PerformanceAnalysis , # [doc = " Type system mapping analysis"] TypeSystemMapping , # [doc = " Memory layout and allocation analysis"] MemoryAnalysis , }
};
}
