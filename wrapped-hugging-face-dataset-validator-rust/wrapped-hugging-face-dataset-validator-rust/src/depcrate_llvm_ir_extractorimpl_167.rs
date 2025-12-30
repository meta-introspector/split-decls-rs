// Generated macro for impl_167 (impl)
macro_rules! Depcrate_llvm_ir_extractorimpl_167 {
() => {
// Module: crate::llvm_ir_extractor
// Provides: {"impl_167"}
// Dependencies: {}
impl LLVMAnalysisPhase { # [doc = " Convert phase to string representation for dataset naming"] pub fn as_str (& self) -> & 'static str { match self { LLVMAnalysisPhase :: IRGeneration => "ir_generation" , LLVMAnalysisPhase :: OptimizationPasses => "optimization_passes" , LLVMAnalysisPhase :: CodeGeneration => "code_generation" , LLVMAnalysisPhase :: PerformanceAnalysis => "performance_analysis" , LLVMAnalysisPhase :: TypeSystemMapping => "type_system_mapping" , LLVMAnalysisPhase :: MemoryAnalysis => "memory_analysis" , } } }
};
}
