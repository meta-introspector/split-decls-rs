// Generated macro for impl_935 (impl)
macro_rules! Depcrate_llvm_diagnosticimpl_935 {
() => {
// Module: crate::llvm::diagnostic
// Provides: {"impl_935"}
// Dependencies: {}
impl < 'll > Diagnostic < 'll > { pub (crate) unsafe fn unpack (di : & 'll DiagnosticInfo) -> Self { use super :: DiagnosticKind as Dk ; unsafe { let kind = super :: LLVMRustGetDiagInfoKind (di) ; match kind { Dk :: InlineAsm => InlineAsm (InlineAsmDiagnostic :: unpackInlineAsm (di)) , Dk :: OptimizationRemark => { Optimization (OptimizationDiagnostic :: unpack (OptimizationRemark , di)) } Dk :: OptimizationRemarkOther => { Optimization (OptimizationDiagnostic :: unpack (OptimizationRemarkOther , di)) } Dk :: OptimizationRemarkMissed => { Optimization (OptimizationDiagnostic :: unpack (OptimizationMissed , di)) } Dk :: OptimizationRemarkAnalysis => { Optimization (OptimizationDiagnostic :: unpack (OptimizationAnalysis , di)) } Dk :: OptimizationRemarkAnalysisFPCommute => { Optimization (OptimizationDiagnostic :: unpack (OptimizationAnalysisFPCommute , di)) } Dk :: OptimizationRemarkAnalysisAliasing => { Optimization (OptimizationDiagnostic :: unpack (OptimizationAnalysisAliasing , di)) } Dk :: OptimizationFailure => { Optimization (OptimizationDiagnostic :: unpack (OptimizationFailure , di)) } Dk :: PGOProfile => PGO (di) , Dk :: Linker => Linker (di) , Dk :: Unsupported => Unsupported (di) , Dk :: SrcMgr => InlineAsm (InlineAsmDiagnostic :: unpackSrcMgr (di)) , _ => UnknownDiagnostic (di) , } } } }
};
}
