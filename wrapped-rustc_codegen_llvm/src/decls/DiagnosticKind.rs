macro_rules! deps {
    () => {
        Linker!();
    };
}

macro_rules! DiagnosticKind {
    () => {
        deps!();
        # [doc = " LLVMRustDiagnosticKind"] # [derive (Copy , Clone)] # [repr (C)] # [allow (dead_code)] pub (crate) enum DiagnosticKind { Other , InlineAsm , StackSize , DebugMetadataVersion , SampleProfile , OptimizationRemark , OptimizationRemarkMissed , OptimizationRemarkAnalysis , OptimizationRemarkAnalysisFPCommute , OptimizationRemarkAnalysisAliasing , OptimizationRemarkOther , OptimizationFailure , PGOProfile , Linker , Unsupported , SrcMgr , }
    };
}

DiagnosticKind!()